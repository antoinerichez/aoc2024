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
    let input_array = input.as_bytes();
    let mut index = 0;
    let mut result = 0;
    let mut enabled = true;

    while index < input_array.len() {
        if match input_array[index] {
            b'd' | b'm' => false,
            _ => true
        } {
            index += 1;
            continue;
        }

        if input_array[index..].starts_with(b"mul(") {
            index += 4;
        } else if input_array[index..].starts_with(b"do()") {
            index += 4;
            enabled = true;
            continue;
        } else if input_array[index..].starts_with(b"don't()") {
            index += 7;
            enabled = false;
            continue;
        } else {
            index += 1;
            continue;
        }

        // We should match on : `digit, digit)` to multiply both **IF IT'S ENABLED**
        let first_number = get_number(input_array, &mut index);

        if input_array[index] != b',' {
            continue;
        }

        index += 1;

        let second_number = get_number(input_array, &mut index);

        if input_array[index] != b')' {
            continue;
        }

        index += 1;

        if enabled {
            result += first_number * second_number;
        }
    }

    result
}

/// Gets a number in char array until we encounter a non-digit ascii character
fn get_number(array: &[u8], index: &mut usize) -> u32 {
    let mut number: u32 = 0;

    while array[*index].is_ascii_digit() {
        number = 10 * number + (array[*index] - b'0') as u32;
        *index += 1;
    }

    number
}