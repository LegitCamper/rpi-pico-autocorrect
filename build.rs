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
    env,
    fmt::{Debug, Display},
    fs::File,
    io::Write,
    path::PathBuf,
};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    write_bktree(
        "tree.rs",
        &mut vec!["why", "how", "the", "moon", "cow", "wow"],
    );

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=dict.txt");
}

// fn create_bktree(out: PathBuf) {
//     let mut contents = String::new();
//     contents.push_str("use hashbrown::HashMap;\n");
//     contents.push_str("use bk_tree::{{BKNode, BKTree}, metrics::Levenshtein};\n");
//     // create macro to construct each BKNode
//     contents.push_str(
//         "macro_rules! make_node {
//     ($($element: ident: $ty: ty),*) => {
//         const $name { $($element: $ty),* }
//     }
// }\n",
//     );

//     let mut tree: BKTree<&str> = BKTree::new(metrics::Levenshtein);
//     tree.add("foo");
//     tree.add("bar");
//     tree.add("baz");
//     tree.add("bup");
//     tree.add("foos");

//     fn add_node<K: Debug + Display>(
//         contents: &mut String,
//         node: &BKNode<K>,
//         children: Option<String>,
//     ) -> String {
//         let key = format!("_{}", node.key.to_string().to_uppercase());
//         // let node = format!("const {}: BKNode<Levenshtein> = BKNode {{ key: \"{}\", children: {}, max_child_distance: None }};\n",key, node.key.to_string(), match children {
//         let node = format!(
//             "make_node(\"{}\", {}, {});\n",
//             key,
//             node.key.to_string(),
//             match children {
//                 Some(c) => c,
//                 None => String::from("HashMap::default()"),
//             }
//         );
//         contents.push_str(node.as_str());
//         key
//     }

//     fn tree_iter<K: Debug + Display>(contents: &mut String, node: &BKNode<K>) {
//         let mut children = vec![];
//         for node in node.children.iter() {
//             children.push(node.1);
//             tree_iter(contents, node.1);
//         }

//         let children = if children.is_empty() {
//             None
//         } else {
//             let mut string = String::from("HashMap {");
//             for child in children {
//                 string.push_str(add_node(contents, child, None).as_str());
//             }
//             string.push_str(" }");
//             Some(string)
//         };
//         add_node(contents, node, children);

//         // make sure to push tld nodes to other vec and push them here
//     }

//     tree_iter(&mut contents, &tree.root.unwrap());

//     File::create(out.join("tree.rs"))
//         .unwrap()
//         .write_all(contents.as_bytes())
//         .unwrap();
// }
