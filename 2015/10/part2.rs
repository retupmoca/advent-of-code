#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    for line in stdin.lines() {
        let line = line.unwrap();
        let mut line = line.trim_end().to_owned();

        for _ in 0..50 {
            line = look_and_say(line);
        }

        println!("Len: {}", line.len());
        break;
    }
}

fn look_and_say(input: String) -> String {
    let mut output = "".to_owned();
    let mut last = ' ';
    let mut last_cnt = 0;
    for ch in input.chars() {
        if ch == last {
            last_cnt += 1;
        }
        else {
            if last_cnt > 0 {
                output += &format!("{}{}", last_cnt, last);
            }
            last_cnt = 1;
        }
        last = ch;
    }

    output += &format!("{}{}", last_cnt, last);

    return output;
}
