//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use embedded_bktree::write_bktree;
use std::{
    convert::AsRef,
    env,
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader, Lines, Write},
    path::{Path, PathBuf},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut dict = Vec::new();
    let lines = lines_from_file("./word_list.txt");
    for line in lines {
        dict.push(line)
    }
    dict.remove(dict.len() - 1);
    let mut dict = dict.iter().map(|w| w.as_str()).collect();

    write_bktree("tree.rs", &mut dict);

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=word_list.txt");
}
