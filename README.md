# rust_link_demo
In this demo repository, you can have a glimpse of bidirectional link between C and Rust with both static library and shared library 

## Create a library by Rust Cargo

Change the Cargo.toml by adding the following statements:

```toml
[lib]
crate-type = ["XXX", "XXX"]; 
```

where "XXX" can be: 

1. lib: create a Rust library
2. staticlib: create a static C library with .a
3. cdylib: create a shared library in Linux or a dynamic link library in Windows
4. bin: generate a binary file

## Some tips

1. You can only compile C code with Rust functions. It is wired that compilation would fail if you were trying to link Rust function to '.cxx' files.
2. You need to add `-ldl -pthread` libraries to basically any Rust-linked C code.
