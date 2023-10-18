#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut overhead = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let line = line.trim_end();
        if line.len() < 2 {
            continue;
        }
        let line: Vec<u8> = line.bytes().collect();
        let mut i = 1;
        while i < (line.len() - 1) {
            if line[i] == b'\\' {
                overhead += 1;
            }
            else if line[i] == b'"' {
                overhead += 1;
            }
            i += 1;
        }
        overhead += 4;
    }
    println!("overhead: {}", overhead);
}
