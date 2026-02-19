My personal [`cargo generate`](https://github.com/cargo-generate/cargo-generate) template,
shamelessy stolen from [rust-github/template](https://github.com/rust-github/template/),
but allows creating workspaces and enables many lints.

Usage:
```sh
cargo generate ShE3py/rust-template
```

To update the lint store:
```sh
./check-lints.sh
```

It will ask for new lint levels as needed, e.g.:
> warn: lint [`invalid_doc_attributes`](https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#invalid-doc-attributes)
> default was relaxed from `deny` to `warn`, while current value is `deny`  
> new value: deny  

To get the updated `.cargo/config.toml`, run (in the `tools/` subfolder):
```sh
cargo run --bin reflag
```
