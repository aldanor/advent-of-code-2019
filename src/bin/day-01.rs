use std::iter;

use aoc19::parse_ints;

fn compute_fuel(mass: i64) -> i64 {
    iter::successors(Some(mass), |&f| if f > 6 { Some(f / 3 - 2) } else { None })
        .skip(1)
        .sum()
}

fn main() {
    let input = include_str!("inputs/day-01.txt");
    let data: Vec<i64> = parse_ints(input, '\n').collect();

    let answer1: i64 = data.iter().cloned().map(|x| (x / 3) - 2).sum();
    println!("{}", answer1);

    let answer2: i64 = data.iter().cloned().map(compute_fuel).sum();
    println!("{}", answer2);
}
