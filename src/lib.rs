use std::collections::BTreeSet;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::str::FromStr;

pub fn parse_ints<'a, T>(string: &'a str, sep: char) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    string.trim().split(sep).map(|s| s.parse().unwrap())
}

pub fn get_overlap<'a1, 'a2, T, I1, I2>(iter1: I1, iter2: I2) -> BTreeSet<T>
where
    T: Clone + Ord + 'a1 + 'a2,
    I1: IntoIterator<Item = &'a1 T>,
    I2: IntoIterator<Item = &'a2 T>,
{
    let set1 = BTreeSet::from_iter(iter1.into_iter().cloned());
    let set2 = BTreeSet::from_iter(iter2.into_iter().cloned());
    set1.intersection(&set2).cloned().collect()
}
