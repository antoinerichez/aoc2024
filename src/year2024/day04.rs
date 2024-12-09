use crate::{
    util::grid::*,
    util::point::*
};

type Input = Grid<u8>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

// Detects XMAS occurences
pub fn part1(input: &Input) -> u32 {
    let width = input.height; // Grid is a square
    let mut result = 0;

    // Horizontal & vertical scanning
    for i in 0..width {
        result += scan_line(input, Point::new(i, 0), DOWN, width);
        result += scan_line(input, Point::new(0, i), RIGHT, width);
    }

    // Diagonal scanning
    for i in 0..width - 3 {
        result += scan_line(input, Point::new(i, 0), DOWN + RIGHT, width - i);
        result += scan_line(input, Point::new(0, i + 1), DOWN + RIGHT, width - 1 - i);
        result += scan_line(input, Point::new(width - 1 - i, 0), DOWN + LEFT, width - i);
        result += scan_line(input, Point::new(width - 1, i + 1), DOWN + LEFT, width - 1 - i);
    }

    result
}

// Detects multiple MAS occurences -> Look for A and then look for diagonals
pub fn part2(input: &Input) -> u32 {
    let mut result = 0;

    for x in 1..input.width - 1 {
        for y in 1..input.height - 1 {
            let point = Point::new(x, y);

            if input[point] == b'A' {
                let left_up = input[Point::new(x - 1, y - 1)];
                let right_up = input[Point::new(x + 1, y - 1)];
                let left_down = input[Point::new(x - 1, y + 1)];
                let right_down = input[Point::new(x + 1, y + 1)];
                // "M" = 77 | "S" = 53 => diff == 6.
                result += (left_up.abs_diff(right_down) == 6 && right_up.abs_diff(left_down) == 6) as u32;
            }
        }
    }

    result
}

// Scans a line in both forward or reverse direction.
fn scan_line(grid: &Input, mut point: Point, direction: Point, size: i32) -> u32 {
    let mut result = 0;
    let mut bytes = 0;

    for _ in 0..size {
        bytes = (bytes << 8) | (grid[point] as u32); // Shift bytes to the left when reading the next character
        point += direction; // Shift the coordinates in the given direction
        result += (bytes == 0x584d4153 || bytes == 0x53414d58) as u32; // Matches on XMAS or SAMX
    }

    result
}