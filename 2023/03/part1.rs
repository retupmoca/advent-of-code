#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct PartNum(Cell<bool>, isize);

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut num_positions: HashMap<(isize, isize), Vec<Rc<PartNum>>> = HashMap::new();
    let mut symbols: Vec<(isize, isize)> = Vec::new();

    let mut lnum = 0;
    for line in stdin.lines() {
        let line = line.unwrap();
        let mut num: isize = -1;
        let mut from_col = -1;

        let mut col = 0;
        for ch in line.chars() {
            if !ch.is_digit(10) && ch != '.' && ch != '\n' {
                symbols.push((lnum, col));
            }

            if ch.is_digit(10) {
                if from_col == -1 {
                    from_col = col;
                }

                if num == -1 {
                    num = ch.to_digit(10).unwrap() as isize;
                }
                else {
                    num = num * 10 + ch.to_digit(10).unwrap() as isize;
                }
            }
            else {
                if num > -1 {
                    update_num_positions(&mut num_positions, num, lnum, from_col, col);

                    // reset
                    from_col = -1;
                    num = -1;
                }
            }

            col += 1;
        }
        if num > -1 {
            update_num_positions(&mut num_positions, num, lnum, from_col, col);

            // reset
            from_col = -1;
            num = -1;
        }

        lnum += 1;
    }

    let mut sum = 0;

    for pos in symbols {
        if let Some(list) = num_positions.get(&pos) {
            for pn in list {
                if pn.0.get() == false {
                    sum += pn.1;
                    pn.0.set(true);
                }
            }
        }
    }

    println!("Result: {}", sum);
}

fn update_num_positions(num_positions: &mut HashMap<(isize, isize), Vec<Rc<PartNum>>>, num: isize, lnum: isize, from_col: isize, col: isize) {
    let partnum = Rc::new(PartNum(Cell::new(false), num));
    let start_col = if from_col > 0 { from_col - 1 } else { from_col };
    // previous line
    let end_col = col;
    if lnum > 0 {
        for i in start_col..=end_col {
            num_positions.entry((lnum - 1, i)).or_insert(Vec::new()).push(Rc::clone(&partnum));
        }
    }
    // current line
    if from_col > 0 {
        num_positions.entry((lnum, start_col)).or_insert(Vec::new()).push(Rc::clone(&partnum));
    }
    num_positions.entry((lnum, end_col)).or_insert(Vec::new()).push(Rc::clone(&partnum));
    // next line
    for i in start_col..=end_col {
        num_positions.entry((lnum + 1, i)).or_insert(Vec::new()).push(Rc::clone(&partnum));
    }
}
