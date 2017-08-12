extern crate gcc;
extern crate cmake;
use std::env;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

macro_rules! t {
    ($e:expr) => (match $e{
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let _ = fs::remove_dir_all(env::var("OUT_DIR").unwrap());
    t!(fs::create_dir_all(env::var("OUT_DIR").unwrap()));
    env::remove_var("DESTDIR");

    let mut cfg = cmake::Config::new("cmark");
    cfg.define("CMARK_STATIC", "ON");
    let dst = cfg.build();
    // println!("{}", dst.display());

    println!("cargo:rustc-flags=-L {}", "build/src");
    println!("cargo:rustc-link-lib=static=cmark-gfm");
    println!("cargo:rustc-link-lib=static=cmark-gfmextensions");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
}
