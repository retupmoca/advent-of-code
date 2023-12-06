#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut time: usize = 0;
    let mut dist: usize = 0;

    for (i, line) in stdin.lines().enumerate() {
        let line = line.unwrap();

        for ch in line.chars() {
            if ch.is_digit(10) {
                if i == 0 {
                    time = time * 10 + ch.to_digit(10).unwrap() as usize;
                }
                else if i == 1 {
                    dist = dist * 10 + ch.to_digit(10).unwrap() as usize;
                }
            }
        }
    }

    let mut ways = 0;
    for t in 1..=time {
        if t * (time - t) > dist {
            ways += 1;
        }
    }

    println!("Result: {}", ways);
}
