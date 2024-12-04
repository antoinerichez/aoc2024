use crate::util::parse::ParseOps;

type Input = (i32, i32);

/// Pour X niveau d'un rapport. On aura un delta X-1.
/// Fonction delta retourne +1 si l'on augmente, -1 si cela descend, 0 si égal et 0 dans les autres cas.
/// Si un rapport contient 5 niveaux. Nous attendons donc un delta = 4
pub fn parse(input: &str) -> Input {
    let mut report = Vec::new();
    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in input.lines() {
        let levels = line.iter_signed::<i32>();
        report.extend(levels);

        let (result_part1, result_part2) = check(&report);

        part_1 += result_part1;
        part_2 += result_part2;

        report.clear();
    }

    (part_1, part_2)
}

/// Si delta = size - 1 => OK pour part1 et part2
/// Sinon on fait les deltas des différences de tous les voisins de chaque pair. Si delta = size - 2 -> OK pour part2
fn check(report: &Vec<i32>) -> (i32, i32) {
    let size = report.len();
    let score: i32 = (1..size).map(|index| delta(report[index], report[index - 1])).sum();

    if score.abs() == (size - 1) as i32 {
        return (1, 1);
    }

    for i in 0..size {
        let mut score = score;

        if i > 0 {
            score -= delta(report[i], report[i - 1]);
        }

        if i < size - 1 {
            score -= delta(report[i + 1], report[i]);
        }

        if i > 0 && i < size - 1 {
            score += delta(report[i + 1], report[i - 1]);
        }

        if score.abs() == (size - 2) as i32 {
            return (0, 1);
        }
    }

    (0, 0)
}


/// +1 si le delta augmente
/// -1 si le delta diminue
/// 0 si égal ou invalide
fn delta(i: i32, j: i32) -> i32 {
    const MAX_INTERVAL: i32 = 3;

    let diff = i - j;

    if diff.abs() <= MAX_INTERVAL {
        diff.signum()
    } else {
        0
    }
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}