#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut total = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let mut linenum = -1;
        let mut lastnum = 0;
        for ch in line.chars() {
            if let Ok(num) = format!("{}", ch).parse::<isize>() {
                lastnum = num;
                if linenum == -1 {
                    linenum = num * 10;
                }
            }
        }
        linenum += lastnum;
        total += linenum;
    }
    println!("Total: {}", total);
}
