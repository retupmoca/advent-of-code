#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut hpos = 0;
    let mut vpos = 0;
    let mut aim = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(" ").collect();
        let dir = split[0];
        let amt: usize = split[1].parse().unwrap();

        match dir {
            "forward" => {
                hpos += amt;
                vpos += aim * amt;
            },
            "up" => aim -= amt,
            "down" => aim += amt,
            _ => unreachable!()
        };
    }

    println!("{}", hpos * vpos);
}
