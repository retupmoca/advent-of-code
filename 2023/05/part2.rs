#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut start: Vec<(usize, usize)> = Vec::new();
    let mut next: Vec<(usize, usize)> = Vec::new();

    let mut got_seeds = false;

    for line in stdin.lines() {
        if !got_seeds {
            let line = line.unwrap();
            if line.len() == 0 { got_seeds = true; continue; }

            let (label, nums) = line.split_once(": ").unwrap();
            let mut nstart = 0;
            for num in nums.split(" ").map(|x| x.parse().unwrap()) {
                if nstart == 0 {
                    nstart = num;
                }
                else {
                    start.push((nstart, nstart + num));
                    next.push((nstart, nstart + num));
                    nstart = 0;
                }
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
            let shift = dstart - sstart;

            for i in 0..start.len() {
                if start[i] == (0, 0) {
                    continue;
                }

                let start_within = start[i].0 >= sstart && start[i].0 < sstart + count;
                let end_within = start[i].1 > sstart && start[i].1 <= sstart + count;
                let superset = start[i].0 < sstart && start[i].1 > sstart + count;

                if start_within && end_within {
                    next[i] = (start[i].0 + shift, start[i].1 + shift);
                }
                else if start_within {
                    // split
                    let over_num = start[i].1 - (sstart + count);
                    start.push((sstart+count, sstart+count+over_num));
                    next.push((sstart+count, sstart+count+over_num));
                    
                    next[i] = (start[i].0 + shift, sstart + count + shift);
                    start[i] = (0, 0);
                }
                else if end_within {
                    // split
                    start.push((start[i].0, sstart));
                    next.push((start[i].0, sstart));
                    
                    next[i] = (sstart + shift, start[i].1 + shift);
                    start[i] = (0, 0);
                }
                else if superset {
                    // split
                    start.push((start[i].0, sstart));
                    next.push((start[i].0, sstart));
                    // split
                    let over_num = start[i].1 - (sstart + count);
                    start.push((sstart+count, sstart+count+over_num));
                    next.push((sstart+count, sstart+count+over_num));
                    
                    next[i] = (sstart + shift, sstart + count + shift);
                    start[i] = (0, 0);
                }
            }
        }
    }

    println!("Result: {}", start.iter().min().unwrap().0);
}
