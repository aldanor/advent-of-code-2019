use std::collections::BTreeMap;
use std::iter;

fn build_map<'a>(input: &'a str) -> Vec<Vec<usize>> {
    let mut known = BTreeMap::new();
    known.insert("COM", 0);
    let mut get_index = |code| -> usize {
        let n = known.len();
        *known.entry(code).or_insert(n)
    };
    let parse_line = |line: &'a str| {
        let line = line.trim();
        let idx = line.find(')').unwrap();
        let (left, right) = (&line[..idx], &line[idx + 1..]);
        (get_index(left), get_index(right))
    };
    let edges = input.trim().lines().map(parse_line).collect::<Vec<_>>();
    let mut map: Vec<_> = iter::repeat_with(Vec::new).take(known.len()).collect();
    edges.iter().cloned().for_each(|(from, to)| {
        map[from].push(to);
    });
    map
}

fn count_orbits(map: &[Vec<usize>], pos: usize, distance: usize) -> usize {
    let count_next = |idx| distance + count_orbits(map, idx, distance + 1);
    map[pos].iter().cloned().map(count_next).sum()
}

fn main() {
    let input = include_str!("inputs/day-06.txt");
    let map = build_map(input);

    let answer1 = count_orbits(&map, 0, 1);
    println!("{}", answer1);
}

#[test]
fn test_count_orbits() {
    let input = "
    COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L
    ";
    assert_eq!(count_orbits(&build_map(input), 0, 1), 42);
}
