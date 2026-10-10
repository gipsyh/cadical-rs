use cmake::Config;
use std::path::{Path, PathBuf};
use std::{env, io};

fn has_cadical_lib(dir: &Path) -> bool {
    ["libcadical.dylib", "libcadical.so", "cadical.dll"]
        .iter()
        .any(|name| dir.join(name).exists())
}

fn cadical_lib_dir() -> io::Result<PathBuf> {
    let dir = PathBuf::from(env::var("CADICAL_DIR").unwrap_or_else(|_| "/usr/local/lib".into()));
    for candidate in [dir.clone(), dir.join("lib")] {
        if has_cadical_lib(&candidate) {
            return Ok(candidate);
        }
    }
    Err(io::Error::other(format!(
        "no libcadical shared library in `{}` or `{}`, point CADICAL_DIR at the directory it is installed in",
        dir.display(),
        dir.join("lib").display()
    )))
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-env-changed=CADICAL_DIR");
    let lib_dir = cadical_lib_dir()?;
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    let bindings = PathBuf::from("./bindings");
    println!("cargo:rerun-if-changed=./bindings");
    println!("cargo:rerun-if-changed=./include");
    Config::new(bindings).build();

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("lib")
            .display()
    );
    println!("cargo:rustc-link-lib=static=bindings");
    println!("cargo:rustc-link-lib=dylib=cadical");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=c++");
    }
    Ok(())
}
