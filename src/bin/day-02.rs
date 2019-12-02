use aoc19::parse_ints;

fn step(data: &mut [usize], i: usize) -> Option<usize> {
    let opcode = data[i];
    if opcode == 99 {
        Some(data[0])
    } else {
        let (i_left, i_right, i_out) = (data[i + 1], data[i + 2], data[i + 3]);
        let (left, right) = (data[i_left], data[i_right]);
        data[i_out] = if opcode == 1 {
            left + right
        } else {
            left * right
        };
        None
    }
}

fn run(data: &[usize], noun: usize, verb: usize) -> usize {
    let mut data = data.to_vec();
    data[1] = noun;
    data[2] = verb;
    (0..data.len())
        .step_by(4)
        .filter_map(|i| step(&mut data, i))
        .next()
        .unwrap()
}

fn main() {
    let input = include_str!("inputs/day-02.txt");
    let data: Vec<_> = parse_ints(input, ',').collect();

    let answer1 = run(&data, 12, 2);
    println!("{}", answer1);
}
