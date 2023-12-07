#!/usr/bin/env rust-script

use std::io::stdin;
use std::io::BufRead;

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Card {
    c: char
}

impl Card {
    fn strength(&self) -> u8 {
        match self.c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => self.c.to_digit(10).unwrap() as u8
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength() > other.strength() {
            return Some(Ordering::Greater);
        }
        if self.strength() < other.strength() {
            return Some(Ordering::Less);
        }

        return Some(Ordering::Equal);
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    bid: usize,
    hand: [Card; 5],
}

impl Hand {
    fn num_matches(&self, card: &Card) -> u8 {
        let mut sum = 0;
        for c in &self.hand {
            if c == card { sum += 1; }
        }
        return sum;
    } 

    fn is_five_kind(&self) -> bool {
        self.num_matches(&self.hand[0]) == 5
    }

    fn is_four_kind(&self) -> bool {
        self.num_matches(&self.hand[0]) == 4
        || self.num_matches(&self.hand[1]) == 4
    }

    fn is_full_house(&self) -> bool {
        let mut has_three = false;
        let mut has_two = false;
        for c in &self.hand {
            if self.num_matches(c) == 3 {
                has_three = true;
            }
            if self.num_matches(c) == 2 {
                has_two = true;
            }
        }

        return has_three && has_two;
    }

    fn is_three_kind(&self) -> bool {
        for c in &self.hand {
            if self.num_matches(c) == 3 {
                return true;
            }
        }

        return false;
    }

    fn is_two_pair(&self) -> bool {
        let mut first_pair = None;

        for c in &self.hand {
            if self.num_matches(c) == 2 {
                if first_pair == None {
                    first_pair = Some(c);
                }
                else if Some(c) != first_pair {
                    return true;
                }
            }
        }

        return false;
    }

    fn is_pair(&self) -> bool {
        for c in &self.hand {
            if self.num_matches(c) == 2 {
                return true;
            }
        }

        return false;
    }

    fn strength(&self) -> u8 {
        if self.is_five_kind() {
            return 7;
        }
        if self.is_four_kind() {
            return 6;
        }
        if self.is_full_house() {
            return 5;
        }
        if self.is_three_kind() {
            return 4;
        }
        if self.is_two_pair() {
            return 3;
        }
        if self.is_pair() {
            return 2;
        }
        return 1;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        if self.strength() > other.strength() {
            return Some(Ordering::Greater);
        }
        if self.strength() < other.strength() {
            return Some(Ordering::Less);
        }

        for i in 0..5 {
            if self.hand[i] > other.hand[i] {
                return Some(Ordering::Greater);
            }
            if self.hand[i] < other.hand[i] {
                return Some(Ordering::Less);
            }
        }

        return Some(Ordering::Equal);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering { self.partial_cmp(other).unwrap() }
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut hands = Vec::new();

    for (i, line) in stdin.lines().enumerate() {
        let line = line.unwrap();
        let (hand, bid_str) = line.split_once(" ").unwrap();
        let bid: usize = bid_str.parse().unwrap();

        let mut chars = hand.chars();

        hands.push(Hand {
            bid,
            hand: [
                Card { c: chars.next().unwrap() },
                Card { c: chars.next().unwrap() },
                Card { c: chars.next().unwrap() },
                Card { c: chars.next().unwrap() },
                Card { c: chars.next().unwrap() },
            ]
        });
    }

    hands.sort();

    let mut sum = 0;
    let mut mul = 1;
    for hand in hands {
        sum += hand.bid * mul;
        mul += 1;
    }
    println!("Result: {}", sum);
}
