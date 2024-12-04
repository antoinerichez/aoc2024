use crate::util::parse::ParseOps;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let mut report = Vec::new();
    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in input.lines() {
        let levels = line.iter_signed::<i32>();
        report.extend(levels);

        part_1 += check(&report, 0) as i32;
        part_2 += check(&report, 1) as i32;

        report.clear();
    }

    (part_1, part_2)
}

fn check(report: &Vec<i32>, fail_safe: i32) -> bool {
    const MIN_INTERVAL: i32 = 1;
    const MAX_INTERVAL: i32 = 3;

    let mut is_safe = true;
    let mut safety: i32 = 0;
    let mut safety_index = 0;
    let decrease_tendency = report[0] > report[1];

    for i in 1..report.len() {
        let first_value = report[i - (1 + (safety_index as usize + safety as usize))];
        let second_value = report[i - safety as usize];
        let factor = (first_value - second_value).abs();

        let is_decreasing = first_value > second_value;
        let is_interval_respected = factor >= MIN_INTERVAL && factor <= MAX_INTERVAL;

        let is_report_safe = decrease_tendency == is_decreasing && is_interval_respected;

        if !is_report_safe {
            safety += 1;
            safety_index += 1;
            is_safe = safety <= fail_safe;
        }

        if !is_safe {
            break
        }
    }

    is_safe
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}