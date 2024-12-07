#![feature(exit_status_error)]
use cmake::Config;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let bindings = PathBuf::from("./bindings");
    println!("cargo:rerun-if-changed=./bindings");
    Config::new(bindings).build();

    println!("cargo:rerun-if-changed=./cadical");
    let cadical_src = PathBuf::from(src_dir).join("cadical");
    let cadical_out = PathBuf::from(&out_dir).join("cadical");
    let num_jobs = env::var("NUM_JOBS").unwrap();
    if cadical_out.exists() {
        fs::remove_dir_all(&cadical_out).unwrap();
    }
    fs::create_dir(&cadical_out).unwrap();
    let overlay = giputils::mount::MountOverlay::new(&cadical_src, &cadical_out);
    Command::new("sh")
        .arg("configure")
        .current_dir(overlay.path())
        .status()
        .map_err(|e| e.to_string())?
        .exit_ok()
        .map_err(|e| e.to_string())?;
    Command::new("make")
        .arg(format!("-j{}", num_jobs))
        .current_dir(overlay.path())
        .status()
        .map_err(|e| e.to_string())?
        .exit_ok()
        .map_err(|e| e.to_string())?;
    println!(
        "cargo:rustc-link-search=native={}",
        cadical_out.join("build").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(out_dir).join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=cadical");
    println!("cargo:rustc-link-lib=static=bindings");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}
