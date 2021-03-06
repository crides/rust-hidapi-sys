extern crate cc;
extern crate pkg_config;

use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if env::var("CARGO_FEATURE_BUILD").is_err() {
        return;
    }

    fetch().expect("failed to checkout hidapi sources, internet connection and git are needed");
    build().expect("failed to build hidapi sources");

    println!(
        "cargo:rustc-link-search=native={}",
        output().to_string_lossy()
    );
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn source() -> PathBuf {
    if let Ok(path) = env::var("HIDAPI_PATH") {
        path.into()
    } else {
        output().join("hidapi")
    }
}

fn fetch() -> io::Result<()> {
    if env::var("HIDAPI_PATH").is_ok() {
        return Ok(());
    }

    Command::new("git")
        .current_dir(&output())
        .arg("clone")
        .arg("https://github.com/crides/hidapi-1.git")
        .arg("hidapi")
        .status()?;

    Ok(())
}

fn build() -> io::Result<()> {
    let mut build = cc::Build::new();

    build.file(source().join("linux/hid.c"));
    build.include(source().join("hidapi"));
    build.static_flag(true);

    for path in pkg_config::find_library("libusb-1.0")
        .unwrap()
        .include_paths
    {
        build.include(path.to_str().unwrap());
    }

    build.compile("libhidapi-hidraw.a");

    Ok(())
}
