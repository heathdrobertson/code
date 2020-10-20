//! I am a comment that documents the code.
//! I use markdown for formating.
//!
//! # Code Examples:
//! ```Rust
//! let name = "Heath";
//! println!("Hello my name is {}", name);

/*
 * I am a multy line commment
 */

// I am a single line comment

extern crate rand;
use rand::Rng;

/// The who am I function `prints` the name of the current Rust file.
fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

pub fn glue() -> bool {
    (true)
}

pub fn main() {
    let mut rng = rand::thread_rng();
    let b:u32 = rng.gen();
    println!("b = {}", b);
    who_am_i();
}
