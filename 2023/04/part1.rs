#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut sum = 0;

    for line in stdin.lines() {
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

        sum += 2_u32.pow(matches - 1);
    }

    println!("Result: {}", sum);
}
