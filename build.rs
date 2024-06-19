//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use bk_tree::{metrics, BKNode, BKTree};
use std::{
    env,
    fmt::{Debug, Display},
    fs::File,
    io::Write,
    path::PathBuf,
};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Generate BkTree dictionary
    create_bktree(out.clone());

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=dict.txt");
}

fn create_bktree(out: PathBuf) {
    let mut contents = String::new();
    contents.push_str("hashbrown::HashMap\n");
    contents.push_str("bk_tree::BKNode\n");
    contents.push_str("bk_tree::metrics::Levenshtein\n");

    let mut tree: BKTree<&str> = BKTree::new(metrics::Levenshtein);
    tree.add("foo");
    tree.add("bar");
    tree.add("baz");
    tree.add("bup");

    fn tree_iter<K: Debug + Display>(file: &mut String, node: &BKNode<K>) {
        for node in node.children.iter() {
            tree_iter(file, node.1);
        }

        file.push_str(format!("const {} BKNode {{ key: {}, children: HashMap::default(), max_child_distance: None }};\n", 
            format!("_{}", node.key.to_string().to_uppercase()), node.key).as_str());

        // make sure to push tld nodes to other vec and push them here
    }

    tree_iter(&mut contents, &tree.root.unwrap());
    File::create(out.join("tree.rs"))
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
}
