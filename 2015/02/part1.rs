#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut total: usize = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('x').collect();
        let l: usize = parts[0].parse().unwrap();
        let w: usize = parts[1].parse().unwrap();
        let h: usize = parts[2].parse().unwrap();

        let sides = [l*w, w*h, h*l];
        let min = sides.iter().min().unwrap();
        let area = 2*(sides[0] + sides[1] + sides[2]);
        total += area + min;
    }

    println!("total: {}", total);
}
