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
        for i in 0..line.len() {
            if let Some(num) = parse_digit(&line[i..]) {
                lastnum = num;
                if linenum == -1 {
                    linenum = num * 10;
                }
            }
        }
        linenum += lastnum;
        println!("{}", linenum);
        total += linenum;
    }
    println!("Total: {}", total);
}

fn parse_digit(s: &str) -> Option<isize> {
    let bytes = s.as_bytes();
    if bytes.len() == 0 {
        return None
    }

    // 0-9
    if bytes[0] >= 0x30 && bytes[0] <= 0x39 {
        return Some((bytes[0] - 0x30).into());
    }

    // names
    let digit_names: [(&[u8], isize); 10] = [
        (b"zero", 0),
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
    ];

    for (name, val) in digit_names {
        if bytes.len() >= name.len() && &bytes[0..name.len()] == name {
            return Some(val);
        }
    }

    // nothing found
    return None
}
