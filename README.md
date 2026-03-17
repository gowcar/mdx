<p align="center">
  <h1 align="center">mdx</h1>
  <p align="center">A fast, beautiful terminal markdown viewer.</p>
</p>

<p align="center">
  <a href="https://github.com/gowcar/mdx/releases"><img src="https://img.shields.io/github/v/release/gowcar/mdx?style=flat-square&color=bd93f9" alt="Release"></a>
  <a href="https://crates.io/crates/mdx-cli"><img src="https://img.shields.io/crates/v/mdx-cli?style=flat-square&color=ff79c6" alt="Crates.io"></a>
  <a href="https://github.com/gowcar/mdx/blob/main/LICENSE"><img src="https://img.shields.io/github/license/gowcar/mdx?style=flat-square&color=50fa7b" alt="License"></a>
</p>

<p align="center">
  <img src="demo.gif" alt="mdx demo" width="720">
</p>

## Install

```bash
# Homebrew
brew install gowcar/tap/mdx

# Cargo
cargo install mdx-cli

# One-line install (macOS & Linux)
curl -fsSL https://raw.githubusercontent.com/gowcar/mdx/main/install.sh | sh
```

## Usage

```bash
mdx README.md
```

## Features

- **Instant launch** — native Rust, starts in milliseconds
- **8 built-in themes** — Dracula, Catppuccin, Nord, Tokyo Night, and more. Press `t` to cycle
- **Custom themes** — fully configurable via `~/.config/mdx/config.toml`
- **Syntax highlighting** — 100+ languages
- **Smart tables** — auto-resize columns, Unicode-aware alignment
- **Vim keybindings** — `j/k`, `gg/G`, `/` search, `n/N` navigate
- **Mouse support** — scroll, drag-select, copy (works in tmux)
- **Hot reload** — edit your file, mdx updates instantly

See [User Guide](docs/user-guide.md) for more details.

## License

[MIT](LICENSE)
