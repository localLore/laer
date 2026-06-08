# code-steps

Notebook-style code step display for Rust. Show syntax-highlighted code in the terminal, step by step, with `[wait]` breakpoints — ideal for examples, tutorials, and educational crates.

## Quick Start

```toml
[dependencies]
code-steps = "0.1"
```

```rust
use code_steps::step;

fn main() {
    step!("Create and save an image", {
        let img = Image::new(128, 128);
        img.save("output.png")?;
        [wait]
    });

    step!("Load it back", {
        let img = Image::load("output.png")?;
        assert_eq!(img.width(), 128);
    });
}
```

<!-- Run with: cargo run --example demo -->

Terminal output:

```
// Create and save an image                          ← cyan comment
   let img = Image::new(128, 128);                  ← syntax-highlighted
   img.save("output.png")?;                          ← ayu-dark theme
   ⏸  [wait] — press any key...                      ← pause
   ok                                                 ← green

// Load it back
   let img = Image::load("output.png")?;
   assert_eq!(img.width(), 128);
   ok
```

## Features

- **`step!` macro** — display and execute a code block as a numbered step
- **`[wait]` breakpoints** — pause execution until a key is pressed
- **Syntax highlighting** — powered by [syntect](https://crates.io/crates/syntect), output to stderr
- **Configurable theme** — switch via `Cargo.toml` metadata, no code changes
- **Zero-dup code** — you write the code once; the macro handles display

## Themes

Set the theme in your **own** `Cargo.toml`:

```toml
[package.metadata.code-steps]
theme = "solarized-dark"
```

Available themes:

| Theme              | key                 | custom file |
|--------------------|---------------------|-------------|
| Ayu Dark           | `ayu-dark`          | bundled     |
| Solarized Dark     | `solarized-dark`    | built-in    |
| Base16 Ocean Dark  | `base16-ocean`      | built-in    |

## API

### `step!` macro

```rust
step!("description", {
    // Rust code
    [wait]  // optional breakpoint
});
```

- `description` — printed as a cyan comment line
- code block — syntax-highlighted, then executed
- `[wait]` — pauses with "press any key..." prompt (optional, can appear multiple times)

### Display functions

All output goes to **stderr**, leaving stdout clean for your program's output.

| Function | Purpose |
|----------|---------|
| `display::print_file_header(path)` | Print a bold file header with surrounding blank lines |
| `display::print_step_header(comment)` | Print a cyan `// comment` line |
| `display::print_code(code)` | Print syntax-highlighted code (saves cursor for dim) |
| `display::dim_code(code)` | Overwrite last code block in gray |
| `display::print_step_done()` | Print green `ok` |
| `display::press_any_key()` | Print yellow prompt and wait for keypress |

## License

MIT OR Apache-2.0
