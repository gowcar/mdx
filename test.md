# Welcome to mdx

A **beautiful** terminal markdown viewer built with Rust.

## Features

Here's what makes mdx special:

- **Gradient headings** — each character gets a unique color
- **Rounded code blocks** — with syntax highlighting
- **Modern tables** — with zebra stripes
- **Vim keybindings** — j/k/gg/G and more
- **Mouse support** — scroll with your trackpad
- **Search** — press `/` to find text

### Code Example

```rust
fn main() {
    println!("Hello from mdx!");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}
```

```python
def greet(name: str) -> str:
    """Generate a greeting message."""
    return f"Hello, {name}! Welcome to mdx."

if __name__ == "__main__":
    print(greet("World"))
```

### Table Example

| Feature | Status | Notes |
|---------|--------|-------|
| Gradient headings | Done | H1/H2 with color interpolation |
| Code blocks | Done | Rounded corners + syntect |
| Tables | Done | Zebra stripes |
| Search | Done | Live search with highlights |
| Mouse scroll | Done | 3-line scroll step |
| File watching | Planned | Phase 2 |
| Image display | Planned | Sixel/Kitty protocol |

### Block Quote

> "The best way to predict the future is to invent it."
> — Alan Kay
>
> > Nested quotes work too!
> > They get a different border color.

### Lists

1. First ordered item
2. Second ordered item
3. Third ordered item

Unordered list:

- Item one
  - Nested item A
  - Nested item B
    - Deep nested item
- Item two
- Item three

---

#### Links

Check out [Rust](https://www.rust-lang.org) and [ratatui](https://ratatui.rs) for more info.

##### Inline Styles

This has **bold**, *italic*, ~~strikethrough~~, and `inline code` styling.

---

*Built with love using Rust + ratatui + comrak + syntect*
