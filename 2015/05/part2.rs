#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut nice_cnt = 0;
    for line in stdin.lines() {
        let mut pairs: HashSet<String> = HashSet::new();
        let line = line.unwrap();
        let line = line.trim_end();
        let mut last_last = ' ';
        let mut last = ' ';
        let mut has_letter_repeat = false;
        let mut has_pairs = false;
        for ch in line.chars() {
            if last_last == ch {
                has_letter_repeat = true;
            }
            if pairs.contains(&format!("{}{}", last, ch)) {
                has_pairs = true;
            }
            pairs.insert(format!("{}{}", last_last, last));
            last_last = last;
            last = ch;
        }
        if has_letter_repeat && has_pairs {
            nice_cnt += 1;
        }
    }

    println!("nice: {}", nice_cnt);
}
