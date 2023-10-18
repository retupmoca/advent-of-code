#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

use std::collections::HashMap;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut values: HashMap<String, u16> = HashMap::new();
    let mut pending: Vec<String> = Vec::new();
    for line in stdin.lines() {
        let line = line.unwrap();
        let line = line.trim_end();
        if line.len() > 0 {
            pending.push(line.to_owned());
        }
    }
    while pending.len() > 0 {
        let mut next_pending: Vec<String> = Vec::new();
        for line in pending {
            if try_execute(&mut values, &line).is_none() {
                next_pending.push(line);
            }
        }
        pending = next_pending;
    }
    println!("{:?}", values);
    println!("");
    println!("a: {}", values.get("a").unwrap());
}

fn try_execute(values: &mut HashMap<String, u16>, command: &str) -> Option<()> {
    let line: Vec<&str> = command.split(' ').collect();

    if line[1] == "->" {
        let input: u16 = line[0].parse().ok().or_else(|| values.get(line[0]).copied())?;
        values.insert(line[2].to_owned(), input);
    }
    else if line[0] == "NOT" {
        let input = values.get(line[1])?;
        values.insert(line[3].to_owned(), !input);
    }
    else {
        let input_a: u16 = line[0].parse().ok().or_else(|| values.get(line[0]).copied())?;
        let input_b: u16 = line[2].parse().ok().or_else(|| values.get(line[2]).copied())?;
        if line[1] == "AND" {
            values.insert(line[4].to_owned(), input_a & input_b);
        }
        else if line[1] == "OR" {
            values.insert(line[4].to_owned(), input_a | input_b);
        }
        else if line[1] == "LSHIFT" {
            values.insert(line[4].to_owned(), input_a << input_b);
        }
        else if line[1] == "RSHIFT" {
            values.insert(line[4].to_owned(), input_a >> input_b);
        }
    }
    Some(())
}
