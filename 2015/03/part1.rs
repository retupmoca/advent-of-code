#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut delivered: HashSet<(isize, isize)> = HashSet::new();
    let mut pos: (isize, isize) = (0, 0);
    delivered.insert(pos);

    for line in stdin.lines() {
        let line = line.unwrap();
        for ch in line.chars() {
            if ch == '>' {
                pos = (pos.0, pos.1 + 1);
            }
            if ch == '<' {
                pos = (pos.0, pos.1 - 1);
            }
            if ch == '^' {
                pos = (pos.0 + 1, pos.1);
            }
            if ch == 'v' {
                pos = (pos.0 - 1, pos.1);
            }
            delivered.insert(pos);
        }
    }

    println!("Delivered: {}", delivered.len());
}
