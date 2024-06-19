//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use bk_tree::{metrics, BKTree};
use build_const::ConstWriter;
use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    // Generate BkTree dictionary
    create_bktree();

    let out = &PathBuf::from(env::var("OUT_DIR").unwrap());

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=dict.txt");
}

fn create_bktree() {
    let mut consts = ConstWriter::for_build("tree").unwrap();
    consts.add_dependency("hashbrown::HashMap");
    consts.add_dependency("bk_tree::metrics::Levenshtein");
    // finish dependencies and starting writing constants
    let mut consts = consts.finish_dependencies();

    let mut tree: BKTree<&str> = BKTree::new(metrics::Levenshtein);
    tree.add("foo");
    tree.add("bar");
    tree.add("baz");
    tree.add("bup");

    if let Some(root) = tree.root {
        consts.add_value("TREE", "BKNode<Levenshtein>", root);
    } else {
        consts.add_value("TREE", "BKTree<Levenshtein>", tree);
    }
}
