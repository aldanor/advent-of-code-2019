use aoc19::parse_ints;

fn main() {
    let input = include_str!("inputs/day-01.txt");
    let answer: i64 = parse_ints::<i64>(input, '\n').map(|x| (x / 3) - 2).sum();
    println!("{}", answer);
}
