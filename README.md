# firefly-types

[ [📄 docs](https://docs.rs/firefly-types/latest/firefly_types/) ] [ [🐙 github](https://github.com/firefly-zero/firefly-types) ] [ [📦 crates.io](https://crates.io/crates/firefly-types) ]

Rust crate for serializing and parsing the [Firefly Zero](https://fireflyzero.com/) metadata file format. It is used by firefly-runtime and firefly-launcher and can be useful if you're making your own launcher.

## Installation

```bash
cargo add firefly-types
```

## Usage

```rust
use firefly_rust::sudo;
use firefly_types::Meta;

let meta_path = "roms/sys/launcher/_meta";
let meta_raw = sudo::load_file_buf(meta_path).unwrap();
let meta = Meta::decode(meta_raw.data()).unwrap();
```

## License

[MIT License](./LICENSE). Feel free to use and modify for any purposes in any apps, commercial or not.
