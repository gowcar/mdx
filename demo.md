# Welcome to mdx

A **fast**, _beautiful_ terminal markdown viewer built with Rust.

## Features

- Gradient colored headings
- **8 built-in themes** — press `t` to cycle
- `syntect` powered syntax highlighting
- Smart tables with auto-resize
- File hot reload on save
- Vim keybindings + mouse support

## Code Example

```rust
fn main() {
    let greeting = "Hello from mdx!";
    println!("{}", greeting);
}
```

```python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a
```

## Comparison

| Tool | Language | Themes | Hot Reload | Syntax Highlight |
|------|----------|--------|------------|------------------|
| **mdx** | Rust | 8 | Yes | Yes |
| glow | Go | 2 | No | Yes |
| mdcat | Rust | 1 | No | No |
| bat | Rust | 20+ | No | Yes |

> "The best way to predict the future is to invent it." — Alan Kay

## Task List

- [x] Gradient headings
- [x] Code blocks with syntax highlighting
- [x] Table auto-resize
- [x] Theme cycling
- [ ] Image display (coming soon)

---

Press `?` for help, `/` to search, `t` to switch themes.
