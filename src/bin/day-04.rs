fn to_digits(num: &str) -> Vec<i32> {
    num.chars().map(|c| c.to_digit(10).unwrap() as _).collect()
}

fn is_valid_1(password: i32) -> bool {
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

fn is_valid_2(password: i32) -> bool {
    let digits = to_digits(&password.to_string());
    let (n_same, n_decreasing) = (1..digits.len())
        .map(|i| {
            (
                digits[i] - digits[i - 1],
                if i >= 2 {
                    Some(digits[i - 1] - digits[i - 2])
                } else {
                    None
                },
                if i < digits.len() - 1 {
                    Some(digits[i + 1] - digits[i])
                } else {
                    None
                },
            )
        })
        .fold((0, 0), |acc, d| {
            let (d, left, right) = d;
            let is_same = d == 0 && left != Some(0) && right != Some(0);
            let (n_same, n_decreasing) = acc;
            (n_same + (is_same as i32), n_decreasing + ((d < 0) as i32))
        });
    n_same > 0 && n_decreasing == 0
}

fn main() {
    let input: (i32, i32) = (152085, 670283);

    let answer1: u32 = (input.0..=input.1).map(|x| is_valid_1(x) as u32).sum();
    println!("{}", answer1);

    let answer2: u32 = (input.0..=input.1).map(|x| is_valid_2(x) as u32).sum();
    println!("{}", answer2);
}
