use zed_extension_api::{
    self as zed, Architecture, LanguageServerId, Os, Result, Worktree,
    settings::LspSettings,
};

const BINARY_NAME: &str = "php-lsp";
const GITHUB_REPO: &str = "jorgsowa/php-lsp";

struct PhpLspExtension {
    cached_binary_path: Option<String>,
}

impl PhpLspExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(path);
        }

        if let Some(cached) = &self.cached_binary_path {
            if std::fs::metadata(cached).is_ok() {
                return Ok(cached.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = zed::current_platform();
        let triple = target_triple(os, arch)?;

        let (asset_name, file_type, binary_name) = match os {
            Os::Windows => (
                format!("{BINARY_NAME}-{triple}.zip"),
                zed::DownloadedFileType::Zip,
                format!("{BINARY_NAME}.exe"),
            ),
            _ => (
                format!("{BINARY_NAME}-{triple}.tar.gz"),
                zed::DownloadedFileType::GzipTar,
                BINARY_NAME.to_string(),
            ),
        };

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset '{asset_name}' in release {}", release.version))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        let binary_path = format!("{version_dir}/{binary_name}");

        if std::fs::metadata(&binary_path).is_err() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download {asset_name}: {e}"))?;

            if os != Os::Windows {
                zed::make_file_executable(&binary_path)?;
            }

            let entries = std::fs::read_dir(".").map_err(|e| e.to_string())?;
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.starts_with(&format!("{BINARY_NAME}-")) && name != version_dir {
                    std::fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

fn target_triple(os: Os, arch: Architecture) -> Result<&'static str> {
    match (os, arch) {
        (Os::Mac, Architecture::Aarch64) => Ok("aarch64-apple-darwin"),
        (Os::Mac, Architecture::X8664) => Ok("x86_64-apple-darwin"),
        (Os::Linux, Architecture::Aarch64) => Ok("aarch64-unknown-linux-gnu"),
        (Os::Linux, Architecture::X8664) => Ok("x86_64-unknown-linux-gnu"),
        (Os::Windows, Architecture::X8664) => Ok("x86_64-pc-windows-msvc"),
        _ => Err(format!("unsupported platform: {os:?} {arch:?}")),
    }
}

impl zed::Extension for PhpLspExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        Ok(settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        Ok(settings.settings)
    }
}

zed::register_extension!(PhpLspExtension);
