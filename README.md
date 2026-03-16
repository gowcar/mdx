# mdx

A **beautiful** terminal markdown viewer built with Rust.

Fast. Fancy. Amazing.

## Features

- **Gradient headings** — H1/H2 with per-character color interpolation
- **Rounded code blocks** — with syntect syntax highlighting (100+ languages)
- **Modern tables** — with zebra stripe rows and auto column width
- **Block quotes** — with colored left border
- **Full search** — `/` to search, `n`/`N` to navigate, live incremental matching
- **Vim keybindings** — `j`/`k`/`gg`/`G`/`f`/`b`/`Ctrl-d`/`Ctrl-u`
- **Mouse support** — scroll with trackpad/mouse wheel
- **4 built-in themes** — Catppuccin, Dracula, Nord, Tokyo Night
- **Configurable** — `~/.config/mdx/config.toml`
- **Tiny binary** — ~4MB statically linked

## Install

### From source

```bash
cargo install --path .
```

### From GitHub Releases

```bash
# macOS (Apple Silicon)
curl -fsSL https://github.com/USER/mdx/releases/latest/download/mdx-aarch64-apple-darwin.tar.gz | tar xz
sudo mv mdx /usr/local/bin/

# macOS (Intel)
curl -fsSL https://github.com/USER/mdx/releases/latest/download/mdx-x86_64-apple-darwin.tar.gz | tar xz
sudo mv mdx /usr/local/bin/

# Linux
curl -fsSL https://github.com/USER/mdx/releases/latest/download/mdx-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv mdx /usr/local/bin/
```

## Usage

```bash
mdx README.md
mdx --theme dracula README.md
echo '# Hello' | mdx
```

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `Ctrl-d` / `Ctrl-u` | Half page down/up |
| `f` / `Space` | Full page down |
| `b` | Full page up |
| `g g` | Go to top |
| `G` | Go to bottom |
| `/` | Search |
| `n` / `N` | Next / previous match |
| `Esc` | Clear search |
| `?` | Toggle help |
| `q` | Quit |
| Mouse wheel | Scroll |
| `Shift` + drag | Select text (terminal native) |

## Configuration

```bash
mdx --init-config    # Create default config
mdx --config-path    # Show config location
```

Config file: `~/.config/mdx/config.toml`

```toml
[general]
theme = "catppuccin"   # catppuccin, dracula, nord, tokyo-night
mouse = true

[keybindings]
quit = ["q", "Ctrl-c"]
scroll_up = ["k", "Up"]
scroll_down = ["j", "Down"]
full_page_down = ["f", "Ctrl-f", "Space"]
full_page_up = ["b", "Ctrl-b"]
```

## Themes

- **Catppuccin Mocha** (default) — warm and cozy
- **Dracula** — purple and pink
- **Nord** — cool blue and green
- **Tokyo Night** — deep blue and purple

Switch with `--theme`:

```bash
mdx --theme dracula README.md
```

## License

MIT
