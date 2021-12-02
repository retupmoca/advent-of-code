#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut nums: Vec<isize> = Vec::new();
    for line in stdin.lines() {
        let line = line.unwrap();
        let num: isize = line.parse().unwrap();
        nums.push(num);
    }

    let mut prev: isize = -1;
    let mut increases = 0;
    for window in nums.windows(3) {
        let num = window.into_iter().sum();
        if prev >= 0 && prev < num { increases += 1; }
        prev = num;
    }
    println!("Increases: {}", increases);
}
