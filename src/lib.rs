pub fn parse_ints<'a>(string: &'a str) -> impl Iterator<Item = i64> + 'a {
    string
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
}
