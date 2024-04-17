use core::panic;
use std::{path::Path, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=src/backend/c/std.c");
    println!("cargo:rerun-if-changed=src/backend/c/std.h");
    println!("cargo:rerun-if-changed=gc.o");
    println!("cargo:rerun-if-changed=compile_gc.sh");

    let target = std::env::var("PROFILE").unwrap();
    let lib = format!("target/{}/lib/", target);
    dbg!(&target);
    let _ = Command::new("mkdir").arg(&lib).spawn().unwrap().wait();
    let _ = Command::new("cp")
        .arg("src/backend/c/std.h")
        .arg(&lib)
        .spawn()
        .unwrap();

    let _ = Command::new("gcc")
        .arg("-c")
        .arg("src/backend/c/std.c")
        .arg("-o")
        .arg(lib.clone() + "runtime.o")
        .spawn()
        .expect("gcc not installed")
        .wait();

    let _ = Command::new("./compile_gc.sh")
        .spawn()
        .expect("failed to spawn script ./compile_gc.sh")
        .wait();
    if !Path::new("gc.o").exists() {
        panic!("failed to compile gc.o");
    }

    let _ = Command::new("cp")
        .arg("gc.o")
        .arg(lib)
        .spawn()
        .expect("failed spawning cp");
}
