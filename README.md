# include_dir_as_map

A procedural macro which embeds files from a directory as a hashmap in the rust
binary. This can be used to embed assets such as images, html, css, and js.

`include_dir_as_map` extends `include_str!()` and `include_bytes!()` and is
similar to [`include_dir`](https://github.com/Michael-F-Bryan/include_dir).

## Usage

### Quickstart

Include the following section in `Cargo.toml`:

```toml
[dependencies]
include_dir_as_map="1"
```

In your rust code, include the following:

```rust
// DirMap is simply an alias for HashMap<String, Vec<u8>>
use include_dir_as_map::{include_dir_as_map, DirMap};

let dirmap: DirMap = include_dir_as_map!("$CARGO_MANIFEST_DIR");
let bytes = dirmap.get("Cargo.toml")?;
```

All paths are relative to the embedded directory, so if `root` contains files
`root/foo.txt` and `root/next/bar.txt`, then `include_dir_as_map!("root")` will
result in a hashmap with keys `foo.txt` and `next/bar.txt`.

### Examples

See the `examples/` directory for more examples:

* [Hello World](examples/example-hello-world)
* [Actix Web](examples/example-actix-web)
