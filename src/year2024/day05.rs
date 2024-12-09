type Input = Vec<char>;

pub fn parse(input: &str) -> Input {
    input.chars().filter(|x| x.is_ascii_digit()).collect()
}

pub fn part1(input: &Input) -> u32 {
    1
}

pub fn part2(input: &Input) -> u32 {
    2
}