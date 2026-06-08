//! Notebook-style code step display for Rust.
//!
//! Show syntax-highlighted code in the terminal, step by step,
//! with `[wait]` breakpoints. Ideal for examples, tutorials,
//! and educational crates.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use code_steps::step;
//!
//! step!("1. Create an image", {
//!     let img = Image::new(128, 128);
//!     img.save("output.png")?;
//!     [wait]
//! });
//! ```
//!
//! # Theme Configuration
//!
//! Set in your `Cargo.toml`:
//!
//! ```toml
//! [package.metadata.code-steps]
//! theme = "ayu-dark"
//! ```
//!
//! Available: `ayu-dark`, `solarized-dark`, `base16-ocean`.

pub mod display;
pub use code_steps_macros::step;
