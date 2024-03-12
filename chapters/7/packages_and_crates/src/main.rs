// crate: the smallest unit of code considered by Rust at compilation. It can come in two forms; a
// library crate or a binary crate.

// binary crate: can be compiled into an executable binary file. It must contain a main function.
// library crate: contains code that can be used in other programs. It doesn't contain a main function.

// package: a bundle of one or more crates which includes a cargo.toml file which describes how to
// compile its included crates. A package can contain as a many binary crates as necessary but only
// one library crate.

// each binary crate can be defined in src/bin where as a library crate can be defined as src/lib.rs
// as there can only be one library crate per package.

use packages_and_crates;

fn main() {
    println!("I'm the crate root and I'm a binary crate!");

    println!("I'm using a library crate!");
    packages_and_crates::print_crate_type();
}

// if you have multiple binaries in a package, rust will need to be told which one to use when compiling
// and executing.

// error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary,
// or the `default-run` manifest key. available binaries: cmd, packages_and_crates

// to fix this, you can use the --bin flag to specify which binary to run:
// cargo run --bin cmd or cargo run --bin packages_and_crates
