#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut nice_cnt = 0;
    'lines: for line in stdin.lines() {
        let line = line.unwrap();
        let line = line.trim_end();
        let mut last = ' ';
        let mut vowel_cnt = 0;
        let mut has_double = false;
        for ch in line.chars() {
            if last == ch {
                has_double = true;
            }
            if "aeiou".contains(ch) {
                vowel_cnt += 1;
            }
            let pair = format!("{}{}", last, ch);
            let pair: &str = &pair;
            if ["ab", "cd", "pq", "xy"].contains(&pair) {
                continue 'lines;
            }
            last = ch;
        }
        if has_double && vowel_cnt >= 3 {
            nice_cnt += 1;
        }
    }

    println!("nice: {}", nice_cnt);
}
