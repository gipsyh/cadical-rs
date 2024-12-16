#![feature(exit_status_error)]
use cmake::Config;
use giputils::build::copy_build;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings = PathBuf::from("./bindings");
    println!("cargo:rerun-if-changed=./bindings");
    Config::new(bindings).build();

    println!("cargo:rerun-if-changed=./cadical");
    let cb_path = copy_build("cadical", |src| {
        Command::new("sh")
            .arg("configure")
            .current_dir(src)
            .status()
            .map_err(|e| e.to_string())?
            .exit_ok()
            .map_err(|e| e.to_string())?;
        let num_jobs = env::var("NUM_JOBS").unwrap();
        Command::new("make")
            .arg(format!("-j{}", num_jobs))
            .current_dir(src)
            .status()
            .map_err(|e| e.to_string())?
            .exit_ok()
            .map_err(|e| e.to_string())
    })?;
    println!(
        "cargo:rustc-link-search=native={}",
        cb_path.join("build").display()
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
