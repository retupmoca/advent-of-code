#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut total = 0;
    let available = (12, 13, 14);

    for line in stdin.lines() {
        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        let line = line.unwrap();
        let mut parts = line.split(": ");
        let mut idparts = parts.next().unwrap().split(" ");
        idparts.next().unwrap();
        let gnum: usize = idparts.next().unwrap().parse().unwrap();

        let draws = parts.next().unwrap().split("; ");
        for draw in draws {
            let cubes = draw.split(", ");
            for cubedraw in cubes {
                let mut cube = cubedraw.split(" ");
                let num: usize = cube.next().unwrap().parse().unwrap();
                let color = cube.next().unwrap();

                if color == "red" {
                    if num > max_red {
                        max_red = num;
                    }
                }
                if color == "green" {
                    if num > max_green {
                        max_green = num;
                    }
                }
                if color == "blue" {
                    if num > max_blue {
                        max_blue = num;
                    }
                }
            }
        }

        if max_red <= available.0 && max_green <= available.1 && max_blue <= available.2 {
            total += gnum;
        }
    }
    println!("Total: {}", total);
}
