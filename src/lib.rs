#![forbid(unsafe_code)]

#![doc = include_str!("../README.md")]

use std::collections::HashMap;

pub use proc_include_dir_as_map::include_dir_as_map;

/// Maps the relative path of each file to its contents as a vector of bytes.
pub type DirMap = HashMap<String, Vec<u8>>;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn valid_macro_call() {
        let dirmap: DirMap = include_dir_as_map!("$CARGO_MANIFEST_DIR/src");
        let _bytes = dirmap.get("lib.rs").unwrap();
    }
}
