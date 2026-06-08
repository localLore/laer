use std::fs;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(theme_ayu_dark)");
    println!("cargo::rustc-check-cfg=cfg(theme_solarized_dark)");
    println!("cargo::rustc-check-cfg=cfg(theme_base16_ocean)");

    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let toml: toml::Value =
        toml::from_str(&fs::read_to_string(format!("{dir}/Cargo.toml")).unwrap()).unwrap();
    let theme = toml["package"]["metadata"]["code-steps"]["theme"]
        .as_str()
        .unwrap_or("ayu-dark");
    println!("cargo:rustc-cfg=theme_{}", theme.replace('-', "_"));
}
