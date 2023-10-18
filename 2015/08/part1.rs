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
            if line[i-1] == b'\\' {
                if line[i] == b'\\' {
                    i += 2;
                    overhead += 1;
                }
                else if line[i] == b'x' {
                    i += 1;
                    overhead += 3;
                }
                else {
                    i += 1;
                    overhead += 1;
                }
            }
            else {
                i += 1;
            }
        }
        overhead += 2;
    }
    println!("overhead: {}", overhead);
}
