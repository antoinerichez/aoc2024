use crate::util::grid::*;
use crate::util::point::*;

type Input = Grid<u8>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

pub fn part1(input: &Input) -> u32 {
    let size = input.width;
    let mut result = 0;

    // Horizontal & vertical scanning
    for i in 0..size {
        result += scan_line(input, Point::new(i, 0), DOWN, size);
        result += scan_line(input, Point::new(0, i), RIGHT, size);
    }

    // Diagonal scanning
    for i in 0..size - 3 {
        result += scan_line(input, Point::new(i, 0), DOWN + RIGHT, size - i);
        result += scan_line(input, Point::new(0, i + 1), DOWN + RIGHT, size - 1 - i);
        result += scan_line(input, Point::new(size - 1 - i, 0), DOWN + LEFT, size - i);
        result += scan_line(input, Point::new(size - 1, i + 1), DOWN + LEFT, size - 1 - i);
    }

    result
}

pub fn part2(input: &Input) -> u32 {
    2
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