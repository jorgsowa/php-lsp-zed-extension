# php-lsp for Zed

Zed extension for [php-lsp](https://github.com/jorgsowa/php-lsp) — a blazing fast PHP language server written in Rust.

## Features

- Diagnostics and error reporting
- Code completion
- Go to definition
- Hover documentation

## Installation

Search for **PHP LSP** in Zed's extension marketplace (`cmd-shift-x`).

The extension downloads the right `php-lsp` binary for your platform on first use. If you already have `php-lsp` in your `PATH`, it uses that instead.

## Configuration

Add settings to your Zed `settings.json` under the `lsp` key:

```json
{
  "lsp": {
    "php-lsp": {
      "initialization_options": {
        "phpVersion": "8.3",
        "excludePaths": ["cache/*", "storage/*"],
        "diagnostics": {
          "deprecatedCalls": false
        }
      }
    }
  }
}
```

| Option | Type | Description |
|---|---|---|
| `phpVersion` | string | Target PHP version. Auto-detected from `composer.json` or system `php` if omitted. |
| `excludePaths` | string[] | Glob patterns to exclude from analysis. |
| `diagnostics` | object | Per-diagnostic toggles. |

## Requirements

Pre-built binaries for macOS, Linux, and Windows — no manual setup.

## License

MIT
