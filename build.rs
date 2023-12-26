use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .expect("Failed to update submodules.");

    let out_dir = env::var("OUT_DIR")
        .map_err(|_| "Environmental variable `OUT_DIR` not defined.".to_string())?;
    let src_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "Environmental variable `CARGO_MANIFEST_DIR` not defined.".to_string())?;

    let cadical_path = PathBuf::from("./bindings");
    println!("cargo:rerun-if-changed=./bindings");
    let mut cfg = Config::new(cadical_path);

    cfg.build();

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(out_dir).join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(src_dir).join("bindings").display()
    );
    println!("cargo:rustc-link-lib=static=bindings");
    println!("cargo:rustc-link-lib=static=cadical");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}
