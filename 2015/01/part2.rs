#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut level = 0;
    let mut pos = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        for ch in line.chars() {
            pos += 1;
            if ch == '(' {
                level += 1;
            }
            if ch == ')' {
                level -= 1;
            }
            if level < 0 {
                println!("Position: {}", pos);
                return;
            }
        }
    }
}
