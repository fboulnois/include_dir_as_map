#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use quote::quote;

#[allow(dead_code)]
type DirMap = HashMap<String, Vec<u8>>;

#[allow(dead_code)]
fn file_to_bytes(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut data)?;
    Ok(data)
}

#[allow(dead_code)]
fn dir_to_map(root: &Path, base: &Path) -> Result<DirMap, Box<dyn std::error::Error>> {
    let mut paths = HashMap::new();
    for entry in std::fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::metadata(&path)?;
        if metadata.is_file() {
            let data = file_to_bytes(&path)?;
            let rel = path.strip_prefix(root)?.to_str().unwrap();
            paths.insert(rel.to_string(), data);
        } else if metadata.is_dir() {
            let dirmap = dir_to_map(root, &path)?;
            paths.extend(dirmap);
        } else {
            panic!("{:?} is not a file or directory", entry);
        }
    }
    Ok(paths)
}

fn env_expand_dir(raw: &str) -> PathBuf {
    let mut copy = raw;
    let mut root = String::new();
    while let Some(pos) = copy.find('$') {
        let (head, tail) = copy.split_at(pos);
        let token = &tail[1..];
        let end = token
            .find(|ch: char| !ch.is_alphanumeric() && ch != '_')
            .unwrap_or(token.len());
        let env = &token[..end];
        let val = std::env::var(env)
            .unwrap_or_else(|_| panic!("{:?} is not a valid environment variable", env));
        copy = token.strip_prefix(env).unwrap();
        root.push_str(head);
        root.push_str(&val);
    }
    root.push_str(copy);
    std::fs::canonicalize(&root).unwrap_or_else(|_| panic!("{:?} is not a valid directory", root))
}

fn strip_quotes(tokens: TokenStream) -> String {
    let mut raw = tokens.to_string();
    if !raw.starts_with('"') || !raw.ends_with('"') || raw.len() < 3 {
        panic!("directory must be a non-empty string");
    }
    raw.pop();
    raw.remove(0);
    raw
}

#[cfg(any(not(debug_assertions), feature = "always-embed"))]
fn internal_dir_as_map(input: TokenStream) -> TokenStream {
    let raw = strip_quotes(input);
    let root = env_expand_dir(&raw);
    let data =
        dir_to_map(&root, &root).unwrap_or_else(|_| panic!("{:?} contains invalid entries", root));

    let it = data.iter().map(|(k, v)| {
        let u = v.iter();
        quote! {
            (String::from(#k), Vec::from([ #( #u ),* ]))
        }
    });

    let output = quote! {
        DirMap::from([ #( #it ),* ])
    };

    output.into()
}

#[cfg(all(debug_assertions, not(feature = "always-embed")))]
fn internal_dir_as_map(input: TokenStream) -> TokenStream {
    let raw = strip_quotes(input);
    let root = env_expand_dir(&raw);
    let root = root
        .to_str()
        .unwrap_or_else(|| panic!("{:?} cannot be converted to utf-8", root));

    let output = quote! {
        {
            use std::{
                collections::HashMap,
                fs::File,
                io::Read,
                path::Path
            };
            fn file_to_bytes(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                let mut data = Vec::new();
                let mut file = File::open(path)?;
                file.read_to_end(&mut data)?;
                Ok(data)
            }
            fn dir_to_map(root: &Path, base: &Path) -> Result<DirMap, Box<dyn std::error::Error>> {
                let mut paths = HashMap::new();
                for entry in std::fs::read_dir(base)? {
                    let entry = entry?;
                    let path = entry.path();
                    let metadata = std::fs::metadata(&path)?;
                    if metadata.is_file() {
                        let data = file_to_bytes(&path)?;
                        let rel = path.strip_prefix(root)?.to_str().unwrap();
                        paths.insert(rel.to_string(), data);
                    } else if metadata.is_dir() {
                        let dirmap = dir_to_map(root, &path)?;
                        paths.extend(dirmap);
                    } else {
                        panic!("{:?} is not a file or directory", entry);
                    }
                }
                Ok(paths)
            }
            dir_to_map(Path::new(#root), Path::new(#root)).unwrap_or_else(|_| panic!("{:?} contains invalid entries", #root))
        }
    };

    output.into()
}

/// The procedural macro magic that embeds files from a directory into the rust
/// binary as a hashmap.
///
/// The input must be a string literal that represents the directory to embed.
/// This string can contain environment variables which will be expanded at
/// compile time.
///
/// The output is a `DirMap` which is an alias for `HashMap<String, Vec<u8>>`.
/// This hashmap maps the relative path of each file to its contents as a vector
/// of bytes.
///
/// By default, the files are read from the filesystem at runtime in debug mode
/// for compilation speed. To override this behavior, enable the `always-embed`
/// feature in `Cargo.toml`.
///
/// # Panics
///
/// This function will panic if the directory does not exist or if any file in
/// the directory cannot be read.
///
/// # Examples
///
/// ```ignore
/// use include_dir_as_map::{include_dir_as_map, DirMap};
///
/// let dirmap: DirMap = include_dir_as_map!("$CARGO_MANIFEST_DIR");
/// let bytes = dirmap.get("Cargo.toml")?;
/// ```
#[proc_macro]
pub fn include_dir_as_map(input: TokenStream) -> TokenStream {
    internal_dir_as_map(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn valid_dir() {
        let root = Path::new(".");
        let _data = dir_to_map(root, root).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_dir() {
        let root = Path::new("invalid_path_for_testing");
        let _data = dir_to_map(root, root).unwrap();
    }

    #[test]
    fn valid_env_var() {
        let path = env_expand_dir("$CARGO_MANIFEST_DIR");
        let ok = path.ends_with("proc");
        assert_eq!(ok, true);
    }

    #[test]
    fn valid_env_var_2() {
        std::env::set_var("SRC_DIR", "src");
        let path = env_expand_dir("$CARGO_MANIFEST_DIR/$SRC_DIR");
        let ok = path.ends_with("src");
        assert_eq!(ok, true);
    }

    #[test]
    #[should_panic]
    fn invalid_env_var() {
        let _path = env_expand_dir("$INVALID_ENV_VAR_FOR_TESTING");
    }
}
