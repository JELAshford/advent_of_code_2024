use std::collections::HashSet;

pub struct Problem {
    rules: HashSet<(isize, isize)>,
    instructions: Vec<Vec<isize>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Problem {
    let problem_parts: Vec<&str> = input.split("\n\n").collect();
    Problem {
        rules: problem_parts[0]
            .lines()
            .map(|line| {
                let vals = line
                    .split("|")
                    .map(|val| val.parse().expect("No map to int"))
                    .collect::<Vec<isize>>();
                (vals[0], vals[1])
            })
            .collect(),
        instructions: problem_parts[1]
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|val| val.parse().expect("No map to int"))
                    .collect::<Vec<isize>>()
            })
            .collect(),
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Problem) -> isize {
    input
        .instructions
        .iter()
        .filter_map(|pages| {
            for pos1 in 0..pages.len() {
                for pos2 in (pos1 + 1)..pages.len() {
                    if input.rules.contains(&(pages[pos2], pages[pos1])) {
                        return None;
                    }
                }
            }
            return Some(pages[pages.len() / 2]);
        })
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Problem) -> isize {
    todo!()
}
