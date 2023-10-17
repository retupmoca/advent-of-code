#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut level = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        for ch in line.chars() {
            if ch == '(' {
                level += 1;
            }
            if ch == ')' {
                level -= 1;
            }
        }
    }
    println!("Level: {}", level);
}
