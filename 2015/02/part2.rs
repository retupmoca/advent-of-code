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

        let bow = l*w*h;
        let wrap =
            if l >= w && l >= h { 2*(w+h) }
            else if w >= l && w >= h { 2*(l+h) }
            else { 2*(w+l) };

        total += bow + wrap;
    }

    println!("total: {}", total);
}
