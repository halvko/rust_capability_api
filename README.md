# Project structure

In this project is 9 different crates, one in each subfolder. In general, they all have a layout like:

```
Cargo.toml
src/
|   (lib|main).rs
```

## Explanation of the different crates

- `prelude`: a small set of shared APIs throughout most of the crates
- `object_capabilities`: the implementation of a small API for I/O operations with object capabilities
- `object_diecup`: a small crate using the API defined in `object_capabilities`
- `object_entrypoint`: a small program using the `object_diecup` with a custom authenticator for the IO capability
- `token_capabilities`: the implementation of a small API for I/O operations with tokens as capabilities
- `token_diecup`: a small crate taking a closure for doing I/O
- `token_entrypoint` is a small program implementing a simple closure using `token_capabilities` to use the `token_diecup` crate
- `facet_external` is a small example of a crate exposing an API taking a closure
- `facet_entrypoint` is a small program using a closure to implement the facet pattern

The main purpose of the `Cargo.toml` files is to specify dependencies. The source code is in the
`.rs` files. All entry points are called {something}\_entrypoint and have a `main.rs` file in their
source.

# Running the code

If a working rust toolchain is installed, these Crates can be run, in their respective
directories running `cargo run`. This builds a debug build of the crate and all of its
dependencies and runs it.

If a rust toolchain is not present, the recommended way of getting it is by going to
(rustup.rs)[https://rustup.rs], and following the instructions.
