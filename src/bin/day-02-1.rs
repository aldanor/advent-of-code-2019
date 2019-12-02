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

fn main() {
    let input = include_str!("inputs/day-02.txt");
    let mut data: Vec<usize> = parse_ints(input, ',').collect();
    data[1] = 12;
    data[2] = 2;
    let answer = (0..data.len())
        .step_by(4)
        .filter_map(|i| step(&mut data, i))
        .next()
        .unwrap();
    println!("{}", answer);
}
