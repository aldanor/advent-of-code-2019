fn to_digits(num: &str) -> Vec<i32> {
    num.chars().map(|c| c.to_digit(10).unwrap() as _).collect()
}

fn is_valid(password: i32) -> bool {
    let digits = to_digits(&password.to_string());
    let (n_same, n_decreasing) =
        (1..digits.len())
            .map(|i| digits[i] - digits[i - 1])
            .fold((0, 0), |acc, d| {
                let (n_same, n_decreasing) = acc;
                (n_same + ((d == 0) as i32), n_decreasing + ((d < 0) as i32))
            });
    n_same > 0 && n_decreasing == 0
}

fn main() {
    let input: (i32, i32) = (152085, 670283);
    let answer1 = (input.0..=input.1)
        .map(is_valid)
        .map(|x| x as i32)
        .sum::<i32>();
    println!("{}", answer1);
}
