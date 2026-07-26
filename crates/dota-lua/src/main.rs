// use syn::File;
use std::{fs, path::Path};

/// test
fn main() {
    let path = Path::new(".").canonicalize().unwrap();
    let path = path.join("dota-lua/src/examples/fireball.rs");
    println!("{:?}", path);
    let source = fs::read_to_string(path).unwrap();

    let file = syn::parse_file(&source).unwrap();

    // println!("{:#?}", file);

    for item in file.items {
        println!("{:?}", item);
    }
}