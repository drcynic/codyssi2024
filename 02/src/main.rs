use std::iter::*;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let p1 = input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx + 1, line == "TRUE"))
        .filter(|(_, v)| *v)
        .fold(0, |a, (idx, _)| a + idx);
    println!("part1: {}", p1);

    let p2 = input
        .trim()
        .lines()
        .map(|line| line == "TRUE")
        .tuples()
        .enumerate()
        .map(|(idx, (l, r))| if idx % 2 == 0 { l && r } else { l || r })
        .filter(|v| *v)
        .count();
    println!("part2: {}", p2);

    let mut input = input.trim().lines().map(|line| line == "TRUE").collect::<Vec<_>>();
    let mut sum = 0;
    while !input.is_empty() {
        sum += input.iter().filter(|v| **v).count();
        input = input
            .iter()
            .tuples()
            .enumerate()
            .map(|(idx, (l, r))| if idx % 2 == 0 { *l && *r } else { *l || *r })
            .collect();
    }
    println!("part3: {}", sum);
}
