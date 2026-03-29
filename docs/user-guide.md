# mdx User Guide

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Keybindings](#keybindings)
- [Themes](#themes)
- [Configuration](#configuration)
- [Integrations](#integrations)
- [Dotfiles Management with Stow](#dotfiles-management-with-stow)
- [Terminal Setup](#terminal-setup)
- [Tips & Best Practices](#tips--best-practices)
- [Troubleshooting](#troubleshooting)

---

## Installation

### Homebrew (macOS & Linux)

```bash
brew install gowcar/tap/mdx
```

### Cargo (from crates.io)

```bash
cargo install mdx-cli
```

The crate is published as `mdx-cli` on crates.io, but the binary is named `mdx`.

### One-line Install Script

For systems without a package manager:

```bash
curl -fsSL https://raw.githubusercontent.com/gowcar/mdx/main/install.sh | sh
```

This downloads the latest release binary for your platform and installs it to `/usr/local/bin/`.

### From GitHub Releases

```bash
# macOS (Apple Silicon)
curl -fsSL https://github.com/gowcar/mdx/releases/latest/download/mdx-aarch64-apple-darwin.tar.gz | tar xz
sudo mv mdx /usr/local/bin/

# macOS (Intel)
curl -fsSL https://github.com/gowcar/mdx/releases/latest/download/mdx-x86_64-apple-darwin.tar.gz | tar xz
sudo mv mdx /usr/local/bin/

# Linux (x86_64)
curl -fsSL https://github.com/gowcar/mdx/releases/latest/download/mdx-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv mdx /usr/local/bin/
```

### Build from Source

```bash
git clone https://github.com/gowcar/mdx.git
cd mdx
cargo build --release
sudo cp target/release/mdx /usr/local/bin/
```

---

## Quick Start

```bash
# View a markdown file
mdx README.md

# Pipe content
echo '# Hello World' | mdx

# Initialize config
mdx --init-config

# Show config location
mdx --config-path

# Use a specific theme
mdx --theme nord README.md
```

---

## Keybindings

### Navigation

| Key | Action |
|-----|--------|
| `j` / `Down` | Scroll down one line |
| `k` / `Up` | Scroll up one line |
| `Ctrl-d` | Half page down |
| `Ctrl-u` | Half page up |
| `f` / `Space` | Full page down |
| `b` | Full page up |
| `g g` | Go to top |
| `G` | Go to bottom |

### Search

| Key | Action |
|-----|--------|
| `/` | Open search bar |
| `Enter` | Confirm search |
| `n` | Next match |
| `N` | Previous match |
| `Esc` | Cancel / clear search |

### Themes

| Key | Action |
|-----|--------|
| `t` | Next theme |
| `T` | Previous theme |

Theme choice is automatically saved and remembered across sessions.

### Text Selection

| Key | Action |
|-----|--------|
| Mouse drag | Select text |
| `y` | Copy selection to clipboard (via OSC 52) |
| `Esc` | Clear selection |

This works correctly inside tmux panes without crossing pane boundaries.

### Other

| Key | Action |
|-----|--------|
| `?` / `h` | Toggle help popup |
| `q` / `Ctrl-c` | Quit |
| Mouse wheel | Scroll up/down |

---

## Themes

mdx ships with 8 built-in themes. Press `t` to cycle through them:

| Theme | Style | Best For |
|-------|-------|----------|
| **Dracula** (default) | Vivid neon gothic | Dark terminals |
| **Catppuccin** | Warm pastel | Easy on the eyes |
| **Nord** | Arctic frost | Calm, professional |
| **Tokyo Night** | Neon city lights | High contrast |
| **Gruvbox** | Retro warm amber | Vintage feel |
| **Solarized** | Scientific precision | Mixed lighting |
| **One Dark** | Clean code elegance | Atom users |
| **Monokai** | High-voltage neon | Sublime Text fans |

Each theme has a unique visual identity including:
- Different list markers (e.g., Dracula uses `◆`, Nord uses `◇`, Monokai uses `▶`)
- Different blockquote border styles
- Different heading separator characters
- Different status bar icons
- Distinct bold/italic colors

Your last selected theme is saved to `~/.config/mdx/last_theme` and automatically loaded on next launch.

### Theme Priority

1. CLI flag: `mdx --theme nord README.md` (highest)
2. Last saved theme: `~/.config/mdx/last_theme`
3. Config file: `theme = "dracula"` in config.toml
4. Default: Dracula

---

## Configuration

### Creating the Config File

```bash
mdx --init-config
# Creates ~/.config/mdx/config.toml
```

### Full Configuration Reference

```toml
[general]
# Available: catppuccin, dracula, nord, tokyo-night, gruvbox, solarized, one-dark, monokai
# Press t/T in viewer to cycle themes
theme = "dracula"
mouse = true
wrap = true
max_width = 120
padding = 2
nerd_font = "auto"   # "auto" | "true" | "false"

[display]
line_numbers_in_code = false
show_language_label = true
rounded_code_blocks = true
table_zebra_stripes = true

[status_bar]
show_time = true
show_percentage = true
show_keyhints = true
show_theme_name = true
time_format = "%H:%M"
progress_style = "both"   # "bar" | "percentage" | "both"

[keybindings]
# Each action maps to an array of key strings
# Uncomment and modify to customize:
# quit = ["q", "Ctrl-c"]
# scroll_up = ["k", "Up"]
# scroll_down = ["j", "Down"]
# page_up = ["Ctrl-u", "PageUp"]
# page_down = ["Ctrl-d", "PageDown"]
# full_page_up = ["b", "Ctrl-b"]
# full_page_down = ["f", "Ctrl-f", "Space"]
# top = ["g g", "Home"]
# bottom = ["G", "End"]
# search = ["/"]
# search_next = ["n"]
# search_prev = ["N"]
# help = ["?", "F1"]
```

### Nerd Font Setting

The `nerd_font` option controls whether mdx uses Nerd Font icons:

- `"auto"` (default) — Detects terminal. Enables for Kitty, WezTerm, Alacritty automatically.
- `"true"` — Always use Nerd Font icons.
- `"false"` — Use Unicode fallback characters (works everywhere).

You can also set `MDX_NERD_FONT=1` as an environment variable.

---

## Dotfiles Management with Stow

If you use [GNU Stow](https://www.gnu.org/software/stow/) to manage your dotfiles:

### Setup

```bash
# 1. Create the stow package
mkdir -p ~/dotfiles/mdx

# 2. Move your config into the stow package
mv ~/.config/mdx/config.toml ~/dotfiles/mdx/

# 3. Create symlinks
cd ~/dotfiles
stow -t ~/.config/mdx mdx
```

### Verify

```bash
ls -la ~/.config/mdx/config.toml
# Should show: ~/.config/mdx/config.toml -> ../../dotfiles/mdx/config.toml
```

### What NOT to Stow

- `~/.config/mdx/last_theme` — This is runtime state (auto-saved when you press `t`), not configuration. Don't include it in your dotfiles.

### Directory Structure

```
~/dotfiles/
└── mdx/
    └── config.toml    # Your mdx configuration
```

---

## Terminal Setup

### Recommended Fonts

mdx looks best with a Nerd Font. Recommended:

```bash
# Install via Homebrew
brew install --cask font-jetbrains-mono-nerd-font
```

Then configure your terminal to use "JetBrainsMono Nerd Font":

| Terminal | Setting |
|----------|---------|
| **iTerm2** | Preferences > Profiles > Text > Font |
| **Alacritty** | `~/.config/alacritty/alacritty.toml` > `[font.normal] family` |
| **Kitty** | `~/.config/kitty/kitty.conf` > `font_family` |
| **WezTerm** | `~/.config/wezterm/wezterm.lua` > `config.font` |

### tmux

For text selection copy to work in tmux, ensure clipboard passthrough is enabled:

```bash
# Add to ~/.tmux.conf
set -g set-clipboard on
```

For best color rendering:

```bash
set -g default-terminal "tmux-256color"
set -ag terminal-overrides ",*:RGB"
```

---

## Tips & Best Practices

### Shell Alias

```bash
# Add to ~/.bashrc, ~/.zshrc, or ~/.config/fish/config.fish
alias m="mdx"
```

### Preview in Git

```bash
# Use mdx as your git pager for markdown
git config --global diff.md.textconv "mdx"
```

### Pipe from Other Commands

```bash
# View command help in mdx
curl -s https://api.github.com/repos/gowcar/mdx | jq -r .body | mdx

# Preview generated markdown
cat CHANGELOG.md | mdx
```

---

## Integrations

### Yazi Integration

mdx can serve as [yazi](https://yazi-rs.github.io/)'s markdown previewer, rendering styled markdown directly in the preview pane.

#### One-command setup

```bash
mdx --setup-yazi
```

This will:
1. Install the `mdx.yazi` previewer plugin to `~/.config/yazi/plugins/mdx.yazi/`
2. Add the previewer config to `~/.config/yazi/yazi.toml`

After setup, navigate to any `.md` file in yazi — the preview pane will show rendered markdown with full styling (colors, code highlighting, tables, etc.).

#### Manual setup

If you prefer to configure manually:

```lua
-- ~/.config/yazi/plugins/mdx.yazi/init.lua
local M = {}

function M:peek(job)
    local child = Command("mdx")
        :args({ "--raw", "-w", tostring(job.area.w), tostring(job.file.url) })
        :stdout(Command.PIPED)
        :stderr(Command.PIPED)
        :spawn()
    if not child then return end

    local output = child:wait_with_output()
    if not output or not output.status or not output.status.success then return end

    local lines = {}
    for line in output.stdout:gmatch("[^\n]*") do
        table.insert(lines, ui.Line.parse(line))
    end

    local offset = job.skip or 0
    local visible = {}
    for i = offset + 1, math.min(#lines, offset + job.area.h) do
        table.insert(visible, lines[i])
    end
    ya.preview_widgets(job, { ui.Text(visible):area(job.area) })
end

function M:seek(job)
    local h = cx.active.current.hovered
    if h then
        local step = job.units > 0 and 1 or -1
        ya.manager_emit("peek", {
            math.max(0, cx.active.preview.skip + step),
            only_if = h.url,
        })
    end
end

return M
```

```toml
# ~/.config/yazi/yazi.toml
[plugin]
prepend_previewers = [
    { mime = "text/markdown", run = "mdx" },
]
```

#### Open with mdx (Enter key)

To also open markdown files with mdx on `Enter`:

```toml
# ~/.config/yazi/yazi.toml
[opener]
markdown = [
    { run = 'mdx "$@"', block = true, desc = "Open with mdx" },
]

[open]
prepend_rules = [
    { mime = "text/markdown", use = "markdown" },
]
```

#### Raw mode

The preview integration uses `--raw` mode, which outputs ANSI-styled text to stdout:

```bash
mdx --raw README.md           # render to stdout
mdx --raw -w 60 README.md     # specify width
cat README.md | mdx --raw     # pipe from stdin
```

This mode is also useful for integration with other tools like fzf preview.

### Hot Reload Workflow

Open your markdown file in your editor and mdx side by side:

```bash
# Terminal 1: Edit
vim README.md

# Terminal 2: Preview (auto-refreshes on save)
mdx README.md
```

---

## Troubleshooting

### Terminal becomes unresponsive after mdx crash

This is usually not mdx — you likely pressed `Ctrl-s` (XON/XOFF flow control). Press `Ctrl-q` to resume.

If mdx did crash, the panic handler should restore your terminal. If it didn't:

```bash
reset
```

### Colors look wrong

Ensure your terminal supports 24-bit color (true color):

```bash
echo -e "\033[38;2;255;100;0mTrue color test\033[0m"
# Should show orange text
```

### Nerd Font icons show as squares

Either install a Nerd Font or set `nerd_font = "false"` in your config.

### Mouse doesn't work in tmux

Make sure mouse is enabled in tmux:

```bash
set -g mouse on
```

### File hot reload not working

Hot reload only works for real files, not stdin input. The file watcher uses a 300ms debounce to avoid excessive refreshes.
