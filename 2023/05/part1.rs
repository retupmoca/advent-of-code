#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut start: Vec<usize> = Vec::new();
    let mut next: Vec<usize> = Vec::new();

    let mut got_seeds = false;

    for line in stdin.lines() {
        if !got_seeds {
            let line = line.unwrap();
            if line.len() == 0 { got_seeds = true; continue; }

            let (label, nums) = line.split_once(": ").unwrap();
            for num in nums.split(" ").map(|x| x.parse().unwrap()) {
                start.push(num);
                next.push(num);
            }
        }
        else {
            let line = line.unwrap();
            if line.contains(":") {
                continue;
            }
            if line.len() == 0 {
                start = next.clone();
                continue;
            }

            let (dstart_str, rest) = line.split_once(" ").unwrap();
            let (sstart_str, count_str) = rest.split_once(" ").unwrap();
            let dstart: usize = dstart_str.parse().unwrap();
            let sstart: usize = sstart_str.parse().unwrap();
            let count: usize = count_str.parse().unwrap();

            for i in 0..start.len() {
                if start[i] >= sstart && start[i] < sstart + count {
                    next[i] = dstart + (start[i] - sstart);
                }
            }
        }
    }

    println!("Result: {:?}", start.iter().min());
}
