use regex::Regex;
use crate::util::parse::ParseOps;

type Input<'h> = &'h str;

pub fn parse(input: &str) -> Input {
    input
}

pub fn part1(input: &Input) -> u32 {
    let regex_validity: Regex = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();

    regex_validity.find_iter(input)
        .map(|line| line.as_str().iter_unsigned::<u32>().collect::<Vec<u32>>())
        .map(|n| n[0] * n[1])
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    2
}