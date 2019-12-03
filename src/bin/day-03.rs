use std::collections::BTreeSet;
use std::iter;

fn find_visited(path: &str) -> BTreeSet<(i64, i64)> {
    path.trim()
        .split(',')
        .map(|s| (s.chars().next().unwrap(), s[1..].parse().unwrap()))
        .flat_map(|(dir, len)| {
            iter::repeat(match dir {
                'U' => (0, 1),
                'R' => (1, 0),
                'D' => (0, -1),
                'L' => (-1, 0),
                _ => unreachable!(),
            })
            .take(len)
        })
        .scan((0, 0), |pos, (dx, dy)| {
            *pos = (pos.0 + dx, pos.1 + dy);
            Some(*pos)
        })
        .collect()
}

fn main() {
    let input = include_str!("inputs/day-03.txt");

    let mut lines = input.lines();
    let visited1 = find_visited(lines.next().unwrap());
    let visited2 = find_visited(lines.next().unwrap());

    let answer1 = visited1
        .intersection(&visited2)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    println!("{}", answer1);
}
