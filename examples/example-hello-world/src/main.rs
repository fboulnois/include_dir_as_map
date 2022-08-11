use include_dir_as_map::{include_dir_as_map, DirMap};

fn main() {
    let dirmap: DirMap = include_dir_as_map!("$CARGO_MANIFEST_DIR");
    let bytes = dirmap.get("src/main.rs").unwrap();
    let main = std::str::from_utf8(bytes).unwrap();
    println!("This file as a string: {:?}", main);
}
