#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut prev: isize = -1;
    let mut increases = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let num: isize = line.parse().unwrap();

        if prev >= 0 && prev < num { increases += 1; }
        prev = num;
    }
    println!("Increases: {}", increases);
}
