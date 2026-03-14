use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let recipes_dir = PathBuf::from(&manifest_dir)
        .join("../../recipes")
        .canonicalize()
        .expect("recipes/ directory not found");

    println!("cargo:rerun-if-changed={}", recipes_dir.display());

    let mut entries: Vec<PathBuf> = fs::read_dir(&recipes_dir)
        .expect("cannot read recipes/")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("toml"))
        .collect();
    entries.sort();

    for p in &entries {
        println!("cargo:rerun-if-changed={}", p.display());
    }

    let includes: String = entries
        .iter()
        .map(|p| format!("    include_str!({:?}),\n", p))
        .collect();

    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("board_tomls.rs");
    fs::write(out, format!("&[\n{}]", includes)).unwrap();
}
