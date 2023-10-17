#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut delivered: HashSet<(isize, isize)> = HashSet::new();
    let mut pos: [(isize, isize); 2] = [(0, 0), (0, 0)];
    delivered.insert(pos[0]);

    let mut idx = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        for ch in line.chars() {
            let pos: &mut (isize, isize) = &mut pos[idx];
            idx = (idx + 1) % 2;
            if ch == '>' {
                pos.1 += 1;
            }
            if ch == '<' {
                pos.1 -= 1;
            }
            if ch == '^' {
                pos.0 += 1;
            }
            if ch == 'v' {
                pos.0 -= 1;
            }
            delivered.insert(pos.clone());
        }
    }

    println!("Delivered: {}", delivered.len());
}
