extern crate autotools;
extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn check_installed(name: &str) -> anyhow::Result<()> {
    let check = Command::new(name).arg("--version").output();

    match check {
        Ok(output) => {
            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "{} is required, but it's not installed or not in PATH.",
                    name
                ));
            }
        }
        Err(_) => {
            return Err(anyhow::anyhow!(
                "{} is required, but it's not installed or not in PATH.",
                name
            ));
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Check if autoconf is installed
    check_installed("autoconf")?;
    check_installed("automake")?;

    let out_dir = env::var("OUT_DIR").map(PathBuf::from)?;
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("jq");

    // See https://github.com/jqlang/jq/tree/jq-1.7.1?#instructions
    autotools::Config::new(&src_dir)
        .reconf("-i")
        .out_dir(&out_dir)
        .with("oniguruma", Some("builtin"))
        .make_args(vec![
            // See https://github.com/jqlang/jq/issues/1936
            "CPPFLAGS=-D_REENTRANT".into(),
        ])
        .build();

    let lib_dir = out_dir.join("lib");
    let include_dir = out_dir.join("include");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:include={}", include_dir.display());

    for lib in &["onig", "jq"] {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    let bindings = bindgen::Builder::default()
        .header("jq/src/jq.h")
        .generate()?;

    bindings.write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}
