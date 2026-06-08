//! Terminal display helpers: syntax highlighting, cursor control, and status output.
//!
//! All output goes to **stderr** to keep stdout clean for program output.
//!
//! The theme is selected at compile time via `[package.metadata.code-steps] theme`
//! in `Cargo.toml` (processed by `build.rs`).

#[cfg(theme_ayu_dark)]
use std::io::Cursor;
use std::io::{self, Write};

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

// ── 主题选择（由 Cargo.toml → build.rs → cfg 控制）──

#[cfg(theme_ayu_dark)]
const THEME: &str = "ayu-dark";
#[cfg(theme_solarized_dark)]
const THEME: &str = "Solarized (dark)";
#[cfg(theme_base16_ocean)]
const THEME: &str = "base16-ocean.dark";

fn highlighting() -> &'static (SyntaxSet, ThemeSet) {
    use std::sync::OnceLock;
    static H: OnceLock<(SyntaxSet, ThemeSet)> = OnceLock::new();
    H.get_or_init(|| {
        let ss = SyntaxSet::load_defaults_newlines();
        #[cfg(theme_ayu_dark)]
        let mut ts = ThemeSet::load_defaults();
        #[cfg(not(theme_ayu_dark))]
        let ts = ThemeSet::load_defaults();

        #[cfg(theme_ayu_dark)]
        {
            let ayu = include_str!("themes/ayu-dark.tmTheme");
            if let Ok(theme) = ThemeSet::load_from_reader(&mut Cursor::new(ayu)) {
                ts.themes.insert("ayu-dark".into(), theme);
            }
        }

        (ss, ts)
    })
}

fn print_highlighted_with_style(code: &str, dim: bool) -> usize {
    let (ss, ts) = highlighting();
    let syntax = ss.find_syntax_by_name("Rust").expect("Rust syntax");
    let theme = &ts.themes[THEME];
    let mut h = HighlightLines::new(syntax, theme);

    let mut line_count = 0;
    for line in LinesWithEndings::from(code) {
        line_count += 1;
        let ranges = h.highlight_line(line, ss).unwrap();
        if dim {
            let _ = write!(io::stderr(), "\x1b[90m   ");
            let _ = writeln!(io::stderr(), "{}", line.trim_end());
            let _ = write!(io::stderr(), "\x1b[0m");
        } else {
            let _ = write!(io::stderr(), "   ");
            for (style, text) in ranges {
                let _ = write!(
                    io::stderr(),
                    "{}",
                    syntect::util::as_24_bit_terminal_escaped(&[(style, text)], false)
                );
            }
        }
    }
    line_count
}

// ── 公开 API ──

pub fn print_file_header(filename: &str) {
    let _ = writeln!(io::stderr());
    let _ = writeln!(io::stderr(), "\x1b[1m──── {} ────\x1b[0m", filename);
    let _ = writeln!(io::stderr());
}

pub fn print_step_header(comment: &str) {
    let _ = writeln!(io::stderr(), "\x1b[36m// {}\x1b[0m", comment);
}

/// 打印语法高亮代码，同时保存光标位置供 dim_code 原地覆盖
pub fn print_code(code: &str) {
    let _ = write!(io::stderr(), "\x1b[s");
    let _ = io::stderr().flush();
    print_highlighted_with_style(code, false);
}

/// 回到 print_code 时的光标位置，灰色覆盖
pub fn dim_code(code: &str) {
    let _ = write!(io::stderr(), "\x1b[u");
    let _ = io::stderr().flush();
    print_highlighted_with_style(code, true);
}

pub fn print_step_done() {
    let _ = writeln!(io::stderr(), "\x1b[32m   ok\x1b[0m\n");
}

pub fn press_any_key() {
    use std::io::Read;
    let _ = writeln!(io::stderr(), "\x1b[33m    ...\x1b[0m");
    let _ = io::stdin().read_exact(&mut [0u8; 1]);
}
