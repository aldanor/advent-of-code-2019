use std::collections::BTreeSet;
use std::iter::{self, FromIterator};

fn find_visited(path: &str) -> Vec<(i64, i64)> {
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

fn manhattan(pos: (i64, i64)) -> i64 {
    pos.0.abs() + pos.1.abs()
}

fn find_overlap<T: Ord + Clone>(x: &[T], y: &[T]) -> Vec<T> {
    let (x, y) = (
        BTreeSet::from_iter(x.iter().cloned()),
        BTreeSet::from_iter(y.iter().cloned()),
    );
    x.intersection(&y).cloned().collect()
}

fn find_index<T: Copy + PartialEq>(x: &[T], val: T) -> usize {
    let is_val = |(i, x): (usize, T)| if x == val { Some(i) } else { None };
    1 + x
        .iter()
        .cloned()
        .enumerate()
        .filter_map(is_val)
        .next()
        .unwrap()
}

fn main() {
    let input = include_str!("inputs/day-03.txt");

    let mut lines = input.lines();
    let visited1 = find_visited(lines.next().unwrap());
    let visited2 = find_visited(lines.next().unwrap());
    let overlap = find_overlap(&visited1, &visited2);

    let answer1 = overlap.iter().cloned().map(manhattan).min().unwrap();
    println!("{}", answer1);

    let signal_delay = |pos| find_index(&visited1, pos) + find_index(&visited2, pos);
    let answer2 = overlap.iter().cloned().map(signal_delay).min().unwrap();
    println!("{}", answer2);
}
