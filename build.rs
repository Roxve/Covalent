use std::process::Command;
fn main() {
    println!("cargo:rerun-if-changed=src/backend/c/std.c");
    println!("cargo:rerun-if-changed=src/backend/c/std.h");
    let target = std::env::var("PROFILE").unwrap();
    let lib = format!("target/{}/lib/", target);
    dbg!(&target);
    let _ = Command::new("mkdir").arg(&lib).spawn().unwrap().wait();
    let _ = Command::new("cp")
        .arg("src/backend/c/std.h")
        .arg(&lib)
        .spawn()
        .unwrap()
        .wait();
    let _ = Command::new("gcc")
        .arg("-c")
        .arg("src/backend/c/std.c")
        .arg("-o")
        .arg(lib.clone() + "runtime.o")
        .spawn()
        .expect("gcc not installed")
        .wait();
}

