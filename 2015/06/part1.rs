#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut lights = [[false; 1000]; 1000];
    for line in stdin.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.trim_end().split(' ').collect();

        if line[0] == "toggle" {
            let mut coords = line[1].split(',');
            let x1:usize = coords.next().unwrap().parse().unwrap();
            let y1:usize = coords.next().unwrap().parse().unwrap();
            let mut coords = line[3].split(',');
            let x2:usize = coords.next().unwrap().parse().unwrap();
            let y2:usize = coords.next().unwrap().parse().unwrap();

            for x in (x1..=x2) {
                for y in (y1..=y2) {
                    lights[x][y] = !lights[x][y];
                }
            }
        }
        else {
            let mut coords = line[2].split(',');
            let x1:usize = coords.next().unwrap().parse().unwrap();
            let y1:usize = coords.next().unwrap().parse().unwrap();
            let mut coords = line[4].split(',');
            let x2:usize = coords.next().unwrap().parse().unwrap();
            let y2:usize = coords.next().unwrap().parse().unwrap();

            let set_to = line[1] == "on";

            for x in (x1..=x2) {
                for y in (y1..=y2) {
                    lights[x][y] = set_to;
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if lights[x][y] {
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}
