use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_ints<'a, T>(string: &'a str, sep: char) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    string.trim().split(sep).map(|s| s.parse().unwrap())
}
