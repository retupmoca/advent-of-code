#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut distances: HashMap<(String, String), usize> = HashMap::new();
    let mut locations: HashSet<String> = HashSet::new();

    for line in stdin.lines() {
        let line = line.unwrap();
        let line = line.trim_end();
        let line: Vec<&str> = line.split(' ').collect();

        let loc_a = line[0].to_owned();
        let loc_b = line[2].to_owned();
        let dist: usize = line[4].parse().unwrap();

        distances.insert((loc_a.clone(), loc_b.clone()), dist);
        distances.insert((loc_b.clone(), loc_a.clone()), dist);
        locations.insert(loc_a);
        locations.insert(loc_b);
    }

    let locations: Vec<String> = locations.into_iter().collect();

    // input is n=8; brute force should be feasible:
    let mut best_dist = usize::MIN;
    let indexes: Vec<usize> = (0..locations.len()).collect();
    for arrangement in permutations(indexes) {
        let dist = get_dist(arrangement, &locations, &distances);
        if dist > best_dist {
            best_dist = dist;
        }
    }

    println!("best dist: {}", best_dist);
}

fn permutations(mut indexes: Vec<usize>) -> Vec<Vec<usize>> {
    // heap's algorithm
    let mut c: Vec<usize> = Vec::new();
    for _ in 0..indexes.len() {
        c.push(0);
    }

    let mut output: Vec<Vec<usize>> = Vec::new();
    output.push(indexes.clone());
    let mut i = 0;
    while i < indexes.len() {
        if c[i] < i {
            if i % 2 == 0 {
                let tmp = indexes[0];
                indexes[0] = indexes[i];
                indexes[i] = tmp;
            }
            else {
                let tmp = indexes[c[i]];
                indexes[c[i]] = indexes[i];
                indexes[i] = tmp;
            }
            output.push(indexes.clone());
            c[i] += 1;
            i = 0;
        }
        else {
            c[i] = 0;
            i += 1;
        }
    }

    return output;
}

fn get_dist(indexes: Vec<usize>, locations: &[String], distances: &HashMap<(String, String), usize>) -> usize {
    let mut dist = 0;
    let mut last = "".to_owned();
    for idx in indexes {
        let cur = locations[idx].clone();
        if last.len() > 0 {
            dist += distances.get(&(last, cur.clone())).unwrap();
        }
        last = cur;
    }
    return dist;
}
