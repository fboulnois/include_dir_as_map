# include_dir_as_map

A procedural macro which embeds files from a directory into the rust binary as
a hashmap. This can be used to embed assets such as images, html, css, and js.

`include_dir_as_map` extends `include_str!()` and `include_bytes!()` and is
similar to [`include_dir`](https://github.com/Michael-F-Bryan/include_dir).

## Usage

Include the following section in `Cargo.toml`:

```toml
[dependencies]
include_dir_as_map="1"
```

In your rust code, include the following:

```rust ignore
// DirMap is an alias for HashMap<String, Vec<u8>>
use include_dir_as_map::{include_dir_as_map, DirMap};

// Environment variables will be expanded at compile time
let dirmap: DirMap = include_dir_as_map!("$CARGO_MANIFEST_DIR");
let bytes = dirmap.get("Cargo.toml")?;
```

All paths are relative to the embedded directory, so if `root` contains files
`root/foo.txt` and `root/next/bar.txt`, then `include_dir_as_map!("root")` will
result in a hashmap with keys `foo.txt` and `next/bar.txt`.

## Features

By default, the files are read from the filesystem at runtime in debug mode for
compilation speed. To override this behavior, enable the `always-embed` feature
in `Cargo.toml`:

```toml
[dependencies]
include_dir_as_map={ version="1", features=[ "always-embed" ] }
```

## Examples

See the `examples/` directory for more examples:

* [Hello World](examples/example-hello-world)
* [Actix Web](examples/example-actix-web)

## Development

### Building

To build the library and examples:

```sh
cargo build --workspace
```

### Testing

To test the library and procedural macro:

```sh
cargo test --workspace
```
