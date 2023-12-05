#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut copies = Vec::new();

    for (i, line) in stdin.lines().enumerate() {
        let line = line.unwrap();
        let (label, data) = line.split_once(":").unwrap();
        let (winning_data, card_data) = data.split_once("|").unwrap();
        let mut winning: Vec<u32> = winning_data
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();
        let mut card: Vec<u32> = card_data
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();

        let mut matches = 0;

        for num in card {
            if winning.contains(&num) {
                matches += 1;
            }
        }

        while copies.len() <= i + matches {
            copies.push(0);
        }
        copies[i] += 1;

        for j in 0..matches {
            copies[i+j+1] += copies[i];
        }
    }

    let mut sum = 0;
    for c in copies {
        sum += c;
    }
    println!("Result: {}", sum);
}
