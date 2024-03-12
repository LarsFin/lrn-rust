// declaring modules
// mod garden;
// the rust compiler will look for the module code in the following order:
// 1. Inside braces on declaration `mod garden { ... }`
// 2. Inside a file named `src/garden.rs`
// 3. Inside a file named `src/garden/mod.rs`

// you can declare submodules in any file other than the crate root, for example you could
// declare `mod vegetables` in `src/garden.rs`, the compiler will search for the module code
// in the following order:
// 1. Inside braces on declaration `mod vegetables { ... }`
// 2. Inside a file named `src/garden/vegetables.rs`
// 3. Inside a file named `src/garden/vegetables/mod.rs`

// modules are private by default so should be prefixed with `pub` to make them public
// the 'use' keyword is used to abreviate the path to a module
// e.g; `use garden::vegetables::carrot;new()` instead of `garden::vegetables::carrot::new()`

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
