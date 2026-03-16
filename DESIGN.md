# mdx — A Beautiful Terminal Markdown Viewer

> Fast. Fancy. Amazing.

## Overview

**mdx** (Markdown eXplorer) is a terminal-based markdown viewer built in Rust, designed to be the most visually stunning and feature-rich markdown reader for the terminal.

## Why mdx?

Existing tools fall short:

| Tool | Language | Missing |
|------|----------|---------|
| Glow | Go | No interactive scrolling, basic visuals |
| mdcat | Rust | No tables, no interactive mode |
| mdless | Ruby | Slow startup, Ruby dependency |
| bleamd | Go | Good features, but visuals could be better |

**mdx fills the gap**: interactive browsing + gorgeous rendering + blazing fast.

---

## Tech Stack

| Component | Choice | Why |
|-----------|--------|-----|
| Language | **Rust** | Fastest startup, smallest binary, zero dependencies |
| TUI Framework | **ratatui + crossterm** | Most active Rust TUI ecosystem, diff-based rendering |
| Markdown Parser | **comrak** | Full AST, GFM compatible (tables, task lists, footnotes) |
| Syntax Highlight | **syntect** | Mature, TextMate grammar support, 100+ languages |
| Image Display | **ratatui-image** | Unified Sixel/Kitty/iTerm2, auto-detection |
| File Watching | **notify** | Cross-platform fs events + debouncing |
| Config | **serde + toml** | Human-friendly config format |
| Gradient Colors | **palette** | Lch color space interpolation (perceptually uniform) |
| CLI Args | **clap** | Derive-based, auto help/completion |

---

## Architecture

```
                    ┌──────────────────────┐
                    │      main.rs         │
                    │  clap → config → App │
                    └──────────┬───────────┘
                               │
                    ┌──────────▼───────────┐
                    │       app.rs         │
                    │   State Machine      │
                    │                      │
                    │  Normal ↔ Search     │
                    │    ↕         ↕       │
                    │  Help    TocSidebar  │
                    └──────────┬───────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
    ┌─────────▼──────┐ ┌──────▼──────┐ ┌───────▼──────┐
    │   event.rs     │ │   ui.rs     │ │  scroll.rs   │
    │                │ │             │ │              │
    │ Key/Mouse      │ │ Layout:     │ │ Virtual      │
    │ FileChange     │ │ Content     │ │ Scrolling    │
    │ Resize         │ │ StatusBar   │ │ Block-based  │
    │ Tick           │ │ Overlays    │ │ height calc  │
    └────────────────┘ └──────┬──────┘ └──────────────┘
                              │
                    ┌─────────▼─────────┐
                    │  render/          │
                    │                   │
                    │  markdown.rs      │  comrak AST → Vec<RenderBlock>
                    │  heading.rs       │  Gradient titles ★
                    │  code_block.rs    │  Rounded borders + syntect ★
                    │  table.rs         │  Zebra stripes
                    │  paragraph.rs     │  Inline styles
                    │  list.rs          │  Bullets/tasks
                    │  blockquote.rs    │  Gradient left border
                    │  link.rs          │  OSC 8 hyperlinks
                    │  image.rs         │  Sixel/Kitty/iTerm2
                    │  hr.rs            │  Gradient fade line
                    └───────────────────┘
```

### Data Flow

```
File (String)
  → comrak::parse_document()     → AST
  → render::markdown::convert()  → Vec<RenderBlock>
  → scroll::viewport()           → visible blocks only
  → ui::draw()                   → ratatui Frame
  → crossterm                    → terminal output
```

---

## Visual Design (Core Differentiator)

### Gradient Headings

```
# Welcome to mdx                    ← Each character a different color
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━     ← Full-width gradient separator

## Getting Started                   ← Softer gradient
── ── ── ── ── ── ──                 ← Half-width dashed line

### Installation                     ← Bold + single accent color
```

Implementation: `palette` crate Lch color space interpolation → per-character `Span` coloring in ratatui.

**Default gradient pairs per theme (Catppuccin Mocha):**
- H1: `#f38ba8` (Red) → `#fab387` (Peach) → `#f9e2af` (Yellow)
- H2: `#cba6f7` (Mauve) → `#f5c2e7` (Pink)
- H3: `#89b4fa` (Blue) solid

### Rounded Code Blocks

```
  ╭─── rust ────────────────────────────────────╮
  │                                             │
  │  fn main() {                                │
  │      println!("Hello, world!");             │
  │  }                                          │
  │                                             │
  ╰─────────────────────────────────────────────╯
```

- Unicode rounded corners: `╭ ╮ ╰ ╯ │ ─`
- Language label top-left with accent color
- Slightly darker background via `Style::bg()`
- syntect highlighting with theme-matched TextMate scheme
- Optional line numbers (dimmed)

### Modern Tables

```
  ┌──────────────┬──────────┬──────────┐
  │  Column A    │ Column B │ Column C │   ← Bold + accent bg
  ├──────────────┼──────────┼──────────┤
  │  value 1     │ value 2  │ value 3  │   ← Row bg #1
  │  value 4     │ value 5  │ value 6  │   ← Row bg #2 (zebra)
  └──────────────┴──────────┴──────────┘
```

- Auto column width calculation
- GFM alignment support (`:---`, `:---:`, `---:`)
- Zebra stripe alternating row backgrounds

### Block Quotes

```
  ▌  "The best way to predict the future
  ▌   is to invent it." — Alan Kay
  ▌
  ▌  ▌  Nested quotes get a different shade
```

- `▌` (U+258C) left border with accent color
- Nested quotes: progressively lighter shades
- Content text slightly dimmed + italic

### Lists

```
  ● First item                     (unordered: filled circle)
    ○ Nested item                  (nested: hollow circle)
      ■ Deep nested                (deep: filled square)

  1. First ordered item
  2. Second ordered item

  ☑ Completed task                 (green checkmark)
  ☐ Pending task                   (dimmed empty box)
```

### Horizontal Rules

```
  ─ ─ ─ ─ ─ ── ─── ──── ─── ── ─ ─ ─ ─ ─
```
Gradient fade: dense in center, fading to edges.

### Images

- **Sixel/Kitty/iTerm2**: Inline image display (auto-detected)
- **Fallback**: Unicode halfblock rendering
- **No support**: `[Image: alt_text]` text placeholder
- Remote images: async download with placeholder

---

## Status Bar Design

```
 📄 README.md  │  📍 42/256  │  ▓▓▓░░░░░░░ 16%  │  🕐 12:34  │  🎨 Catppuccin  │  ↑↓ j/k  /🔍  ?❓
```

### Elements (left to right)

| Icon | Content | Description |
|------|---------|-------------|
| 📄 | Filename | Current file name |
| 📍 | Line/Total | Current position `42/256` |
| 📊 | Progress | Visual progress bar `▓▓▓░░░░░░░ 16%` |
| 🕐 | Time | Current time `HH:MM` |
| 🎨 | Theme | Active theme name |
| Hints | `↑↓ j/k  /🔍  ?❓` | Key binding hints |

### Progress Bar Styles

```toml
# Option A: Bar + percentage
progress_style = "both"     # ▓▓▓░░░░░░░ 16%

# Option B: Bar only
progress_style = "bar"      # ▓▓▓░░░░░░░

# Option C: Percentage only
progress_style = "percentage"  # 16%
```

### Status Bar Configuration

```toml
[status_bar]
show_time = true
show_percentage = true
show_keyhints = true
show_theme_name = true
time_format = "%H:%M"
progress_style = "both"
```

---

## Key Bindings (Vim-style, Fully Customizable)

### Default Bindings

| Key | Action | Config Key |
|-----|--------|------------|
| `j` / `↓` | Scroll down | `scroll_down` |
| `k` / `↑` | Scroll up | `scroll_up` |
| `Ctrl-d` | Half page down | `page_down` |
| `Ctrl-u` | Half page up | `page_up` |
| `f` / `Ctrl-f` / `Space` | Full page down | `full_page_down` |
| `b` / `Ctrl-b` | Full page up | `full_page_up` |
| `g g` | Go to top | `top` |
| `G` | Go to bottom | `bottom` |
| `/` / `Ctrl-f` | Start search | `search` |
| `n` | Next match | `search_next` |
| `N` | Previous match | `search_prev` |
| `Escape` | Clear search / Cancel | `cancel` |
| `q` / `Ctrl-c` | Quit | `quit` |
| `?` / `F1` | Toggle help | `help` |
| `Enter` / `o` | Open link under cursor | `open_link` |
| `r` | Reload file | `reload` |
| `t` | Toggle TOC sidebar | `toc` |
| Mouse wheel | Scroll page | (always active) |
| Mouse click link | Open in browser | (always active) |
| `Shift` + Mouse drag | Select text & copy | Terminal native passthrough |

### Keybinding Configuration

```toml
# ~/.config/mdx/config.toml

[keybindings]
# Each action maps to an array of key strings
quit = ["q", "Ctrl-c"]
scroll_up = ["k", "Up"]
scroll_down = ["j", "Down"]
page_up = ["Ctrl-u", "PageUp"]
page_down = ["Ctrl-d", "PageDown"]
full_page_up = ["b", "Ctrl-b"]
full_page_down = ["f", "Ctrl-f", "Space"]
top = ["g g", "Home"]              # Multi-key sequence supported
bottom = ["G", "End"]
search = ["/"]
search_next = ["n"]
search_prev = ["N"]
cancel = ["Escape"]
help = ["?", "F1"]
open_link = ["Enter", "o"]
reload = ["r"]
toc = ["t"]

# Key format:
#   Single char:  "j", "k", "G", "/", "?"
#   Arrow keys:   "Up", "Down", "Left", "Right"
#   Special:      "Space", "Enter", "Escape", "Tab"
#   Page keys:    "PageUp", "PageDown", "Home", "End"
#   Function:     "F1" - "F12"
#   Ctrl combo:   "Ctrl-f", "Ctrl-c", "Ctrl-d"
#   Multi-key:    "g g" (space-separated sequence, 500ms timeout)
```

### Key Sequence Engine

Multi-key sequences (like Vim's `gg`):
- 500ms timeout between keys
- Visual indicator in status bar showing pending key: `g_`
- Conflict detection at startup: warns if a single key conflicts with a sequence prefix

---

## Theme System

### Built-in Themes

| Theme | H1 Gradient | Code BG | Accent |
|-------|-------------|---------|--------|
| **Catppuccin Mocha** (default) | Red→Peach→Yellow | `#1e1e2e` | `#cba6f7` |
| **Dracula** | Purple→Pink | `#282a36` | `#bd93f9` |
| **Nord** | Frost→Aurora | `#2e3440` | `#88c0d0` |
| **Tokyo Night** | Blue→Purple | `#1a1b26` | `#7aa2f7` |

### Theme Configuration

```toml
# ~/.config/mdx/config.toml

[theme]
name = "catppuccin"   # or "dracula", "nord", "tokyo-night", "custom"

# Override specific colors (optional)
[theme.colors]
heading1_gradient = ["#f38ba8", "#fab387", "#f9e2af"]
heading2_gradient = ["#cba6f7", "#f5c2e7"]
heading3 = "#89b4fa"
code_bg = "#1e1e2e"
code_border = "#45475a"
code_label = "#a6e3a1"
table_header_bg = "#313244"
table_row_alt_bg = "#1e1e2e"
blockquote_border = "#cba6f7"
link = "#89b4fa"
link_url = "#6c7086"
bold = "#cdd6f4"
italic = "#cdd6f4"
strikethrough = "#6c7086"
list_marker = "#f9e2af"
task_checked = "#a6e3a1"
task_unchecked = "#6c7086"
hr = "#45475a"
search_current_bg = "#f9e2af"
search_match_bg = "#585b70"
status_bar_bg = "#181825"
status_bar_fg = "#cdd6f4"
```

### Custom Theme File

Users can create `~/.config/mdx/themes/my-theme.toml`:

```toml
name = "My Custom Theme"

[colors]
heading1_gradient = ["#ff0000", "#00ff00"]
# ... full color specification
```

---

## Configuration (~/.config/mdx/config.toml)

### Full Example

```toml
[general]
theme = "catppuccin"
mouse = true
wrap = true
max_width = 120              # Max content width (0 = terminal width)
padding = 2                  # Horizontal padding
heading_spacing = 1          # Extra blank lines around headings

[display]
line_numbers_in_code = false
show_language_label = true
rounded_code_blocks = true
table_zebra_stripes = true
image_protocol = "auto"      # "auto" | "sixel" | "kitty" | "iterm2" | "block" | "none"

[status_bar]
show_time = true
show_percentage = true
show_keyhints = true
show_theme_name = true
time_format = "%H:%M"
progress_style = "both"      # "bar" | "percentage" | "both"

[keybindings]
quit = ["q", "Ctrl-c"]
scroll_up = ["k", "Up"]
scroll_down = ["j", "Down"]
page_up = ["Ctrl-u", "PageUp"]
page_down = ["Ctrl-d", "PageDown"]
top = ["g g", "Home"]
bottom = ["G", "End"]
search = ["/"]
search_next = ["n"]
search_prev = ["N"]
help = ["?", "F1"]

[watcher]
enabled = true
debounce_ms = 300

[theme]
name = "catppuccin"
```

### CLI Usage

```bash
# Basic usage
mdx README.md

# With options
mdx --theme dracula README.md
mdx --no-mouse README.md
mdx --width 80 README.md

# Initialize default config
mdx --init-config

# Show config path
mdx --config-path

# Read from stdin
cat README.md | mdx
echo "# Hello" | mdx

# Version
mdx --version
```

---

## File Watching & Hot Reload

```
File changed → notify event → 300ms debounce → re-parse AST
  → regenerate RenderBlocks → preserve scroll position → redraw
```

**Scroll position preservation strategy:**
1. Before reload: record the heading anchor or block index at viewport top
2. After reload: find closest matching anchor and scroll to it
3. If no match: keep same line offset (clamped to new content height)

---

## Project Structure

```
mdx/
├── Cargo.toml
├── LICENSE (MIT)
├── DESIGN.md                    # This file
├── .github/
│   └── workflows/
│       └── release.yml          # Cross-platform release builds
├── src/
│   ├── main.rs                  # CLI entry (clap)
│   ├── app.rs                   # App state machine + event loop
│   ├── event.rs                 # Unified event system
│   ├── ui.rs                    # Top-level UI layout
│   ├── render/
│   │   ├── mod.rs               # RenderBlock enum + traits
│   │   ├── markdown.rs          # comrak AST → Vec<RenderBlock>
│   │   ├── heading.rs           # Gradient headings ★
│   │   ├── code_block.rs        # Rounded code blocks + syntect ★
│   │   ├── table.rs             # Modern tables
│   │   ├── list.rs              # Lists + task lists
│   │   ├── blockquote.rs        # Quote blocks
│   │   ├── paragraph.rs         # Paragraphs + inline styles
│   │   ├── link.rs              # OSC 8 hyperlinks
│   │   ├── image.rs             # Sixel/Kitty/iTerm2 images
│   │   └── hr.rs                # Horizontal rules
│   ├── config.rs                # Config loading & merging
│   ├── theme/
│   │   ├── mod.rs               # Theme system
│   │   ├── gradient.rs          # Gradient color engine (Lch)
│   │   └── builtin.rs           # Built-in themes
│   ├── search.rs                # Search + highlight
│   ├── scroll.rs                # Virtual scrolling engine
│   ├── watcher.rs               # File watch hot reload
│   └── widgets/
│       ├── mod.rs
│       ├── status_bar.rs        # Bottom status bar
│       ├── help_popup.rs        # Help overlay
│       └── search_bar.rs        # Search input
└── tests/
    └── fixtures/*.md
```

---

## Implementation Phases

### Phase 1 — MVP (Core rendering + scrolling + visual wow)

1. `cargo init` + Cargo.toml with all dependencies
2. `main.rs`: clap CLI argument parsing
3. `app.rs`: crossterm + ratatui event loop skeleton
4. `event.rs`: keyboard/mouse/resize events
5. `render/markdown.rs`: comrak AST → RenderBlock conversion
6. `render/heading.rs`: **Gradient headings** (core selling point)
7. `render/code_block.rs`: **Rounded borders + syntect highlighting**
8. `render/paragraph.rs`: bold/italic/inline code/links
9. `render/list.rs`: ordered/unordered lists
10. `render/hr.rs`: horizontal rules
11. `scroll.rs`: block-height-based virtual scrolling
12. `theme/`: default Catppuccin Mocha theme
13. `widgets/status_bar.rs`: filename + progress + time + key hints

### Phase 2 — Feature Complete

1. Table rendering (zebra stripes + auto column width)
2. Block quotes (gradient left border)
3. OSC 8 hyperlinks + mouse click to open
4. Search (`/` + `n`/`N` + highlight)
5. Config system (`~/.config/mdx/config.toml`)
6. More built-in themes (Dracula, Nord, Tokyo Night)
7. File watch hot reload (notify + debouncer)
8. Help popup overlay
9. Customizable keybindings

### Phase 3 — Advanced

1. Image display (ratatui-image)
2. Task lists, footnotes
3. Stdin pipe input
4. TOC sidebar
5. GitHub Actions release pipeline
6. Cross-platform testing

---

## Competitive Advantage

| Feature | Glow | mdcat | mdless | bleamd | **mdx** |
|---------|------|-------|--------|--------|---------|
| Interactive browsing | Yes | No | No | Yes | **Yes** |
| Gradient headings | No | No | No | No | **Yes** ★ |
| Rounded code blocks | No | No | No | No | **Yes** ★ |
| Tables | Yes | No | Yes | Yes | **Yes (modern)** |
| Inline images | No | Partial | No | Dither | **Sixel/Kitty** ★ |
| Search | No | No | No | Yes | **Yes** |
| File hot reload | No | No | No | No | **Yes** ★ |
| Vim keybindings | Partial | N/A | No | Yes | **Full + custom** |
| Mouse support | Yes | N/A | No | Yes | **Yes** |
| OSC 8 links | No | Yes | No | Yes | **Yes** |
| Custom themes | Yes | No | No | Yes | **Yes (TOML)** |
| Startup speed | Fast | Fast | Slow | Fast | **Fastest (Rust)** |
| Status bar | Basic | N/A | N/A | Basic | **Rich (emoji+bar)** ★ |

---

*Built with Rust. Made to be beautiful.*
