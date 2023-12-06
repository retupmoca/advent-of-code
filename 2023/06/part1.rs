#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut times: Vec<u16> = Vec::new();
    let mut distances: Vec<u16> = Vec::new();

    for (i, line) in stdin.lines().enumerate() {
        let line = line.unwrap();
        let (label, rest) = line.split_once(":").unwrap();
        let nums: Vec<u16> = rest
            .split(" ")
            .map(|c| c.trim())
            .filter(|c| c.len() > 0)
            .map(|c| c.parse().unwrap())
            .collect();

        if i == 0 {
            times = nums;
        }
        else if i == 1 {
            distances = nums;
        }
    }

    let mut product = 1;
    for race in 0..times.len() {
        let d = distances[race];
        let mut ways = 0;
        for t in 1..=times[race] {
            if t * (times[race] - t) > d {
                ways += 1;
            }
        }

        product *= ways;
    }

    println!("Result: {}", product);
}
