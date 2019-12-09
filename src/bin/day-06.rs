use std::collections::BTreeMap;
use std::iter;

use aoc19::get_overlap;

fn build_map<'a>(input: &'a str, fixed: &[&'static str]) -> Vec<Vec<usize>> {
    let mut known = BTreeMap::<&'a str, usize>::new();
    fixed.iter().enumerate().for_each(|(i, code)| {
        known.insert(code, i);
    });
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

fn reverse_map(map: &[Vec<usize>]) -> Vec<usize> {
    let mut out: Vec<_> = iter::repeat(0).take(map.len()).collect();
    map.iter()
        .enumerate()
        .for_each(|(i, js)| js.iter().for_each(|&j| out[j] = i));
    out
}

fn path_back(rev_map: &[usize], start: usize) -> Vec<usize> {
    let step_back = |pos: &usize| {
        let parent = rev_map[*pos];
        if parent == *pos {
            None
        } else {
            Some(parent)
        }
    };
    iter::successors(Some(start), step_back).collect()
}

fn min_distance(map: &[Vec<usize>], pos1: usize, pos2: usize) -> usize {
    let rev_map = reverse_map(&map);
    let path1 = path_back(&rev_map, pos1);
    let path2 = path_back(&rev_map, pos2);
    let mut distances: Vec<_> = iter::repeat(0).take(rev_map.len()).collect();
    let mut update_distances = |path: &[usize]| {
        let f = |(i, p)| distances[p] += i;
        path.iter().cloned().enumerate().for_each(f);
    };
    update_distances(&path1);
    update_distances(&path2);
    get_overlap(&path1, &path2)
        .iter()
        .map(|&i| distances[i])
        .min()
        .unwrap()
        - 2
}

fn main() {
    let input = include_str!("inputs/day-06.txt");
    let map = build_map(input, &["COM", "YOU", "SAN"]);

    let answer1 = count_orbits(&map, 0, 1);
    println!("{}", answer1);

    let answer2 = min_distance(&map, 1, 2);
    println!("{}", answer2);
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
    assert_eq!(count_orbits(&build_map(input, &["COM"]), 0, 1), 42);
}

#[test]
fn test_min_distance() {
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
    K)YOU
    I)SAN
    ";
    assert_eq!(
        min_distance(&build_map(input, &["COM", "YOU", "SAN"]), 1, 2),
        4
    );
}
