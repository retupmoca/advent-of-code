#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut lights = [[0; 1000]; 1000];
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
                    lights[x][y] += 2;
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

            for x in (x1..=x2) {
                for y in (y1..=y2) {
                    if line[1] == "on" {
                        lights[x][y] += 1;
                    }
                    else if lights[x][y] > 0 {
                        lights[x][y] -= 1;
                    }
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            count += lights[x][y];
        }
    }

    println!("count: {}", count);
}
