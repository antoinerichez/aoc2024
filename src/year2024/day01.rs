use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    input.iter_unsigned::<u32>().chunk::<2>().map(|[l, r]| (l, r)).unzip()
}

pub fn part1(input: &Input) -> u32 {
    let mut left = input.0.clone();
    left.sort_unstable();

    let mut right = input.1.clone();
    right.sort_unstable();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn part2(input: &Input) -> u32 {
    let (left, right) = input;

    let mut freq = FastMap::with_capacity(left.len());
    right.iter().for_each(|r| *freq.entry(*r).or_insert(0) += 1);

    left.iter().filter_map(|l| freq.get(l).map(|o| l * o)).sum()
}