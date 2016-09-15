use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("clang").args(&["src/extlib.c", "-c", "-o"])
        .arg(&format!("{}/libext.o", out_dir))
        .status().unwrap();

    Command::new("ar").args(&(["crus", "libext.a", "libext.o"]))
                        .current_dir(&Path::new(&out_dir))
                        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ext");
}
