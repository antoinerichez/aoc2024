type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let input_array = input.as_bytes();
    let mut index = 0;
    let mut part1 = 0;
    let mut part2 = 0;
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

        let product = first_number * second_number;

        part1 += product;
        if enabled {
            part2 += product;
        }
    }

    (part1, part2)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
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