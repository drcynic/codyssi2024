use std::iter::*;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let values = input.trim().lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let sum = values.iter().fold(0, |a, n| a + n);
    println!("part1: {}", sum);

    let sum = values.iter().sorted().take(values.len() - 20).fold(0, |a, n| a + n);
    println!("part2: {}", sum);

    let sum = values
        .into_iter()
        .enumerate()
        .fold(0, |a, (idx, n)| a + if idx % 2 == 0 { n } else { -n });
    println!("part3: {}", sum);
}
