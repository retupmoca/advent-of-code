#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! md5 = "0.7"
//! ```

use std::io::stdin;

fn main() {
    let input = stdin().lines().next().unwrap().unwrap();
    let input = input.trim_end();

    let mut num = 1;
    while !check_hash(input, num) {
        num += 1;
    }

    println!("num: {}", num);
}

fn check_hash(prefix: &str, num: usize) -> bool {
    let to_hash = format!("{}{}", prefix, num);
    let hashed = format!("{:x}", md5::compute(to_hash));
    return hashed.starts_with("00000");
}
