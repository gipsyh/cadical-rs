use cmake::Config;
use giputils::build::copy_build;
use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

fn main() -> io::Result<()> {
    giputils::build::git_submodule_update()?;
    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings = PathBuf::from("./bindings");
    println!("cargo:rerun-if-changed=./bindings");
    Config::new(bindings).build();

    println!("cargo:rerun-if-changed=./cadical");
    let cb_path = copy_build("cadical", |src| {
        let status = Command::new("sh")
            .arg("configure")
            .env("CXX", "clang++")
            .env("CXXFLAGS", "-fPIC")
            .current_dir(src)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "configure failed with status: {}",
                status
            )));
        }
        let num_jobs = env::var("NUM_JOBS").unwrap();
        let status = Command::new("make")
            .arg(format!("-j{num_jobs}"))
            .current_dir(src)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "make failed with status: {}",
                status
            )));
        }
        Ok(())
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
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
    Ok(())
}
