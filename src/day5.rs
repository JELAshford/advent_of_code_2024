use hashbrown::{HashMap, HashSet};

#[derive(Clone)]
pub struct Problem {
    instructions: Vec<Vec<isize>>,
    sorted_instructions: Vec<Vec<isize>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Problem {
    let problem_parts: Vec<&str> = input.split("\n\n").collect();
    let mut rules: HashMap<isize, HashSet<isize>> = HashMap::new();
    for line in problem_parts[0].lines() {
        let (from, to) = line.split_once("|").unwrap();
        rules
            .entry(from.parse().unwrap())
            .or_default()
            .insert(to.parse().unwrap());
    }
    let instructions: Vec<Vec<isize>> = problem_parts[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|val| val.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();
    let sorted_instructions: Vec<Vec<isize>> = instructions
        .iter()
        .map(|pages| {
            let mut sorted_pages = pages.clone();
            sorted_pages
                .sort_by(|v1, v2| rules.entry(v2.clone()).or_default().contains(v1).cmp(&true));
            sorted_pages
        })
        .collect();

    Problem {
        instructions,
        sorted_instructions,
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Problem) -> isize {
    let new_input = input.clone();
    std::iter::zip(new_input.instructions, new_input.sorted_instructions)
        .filter_map(|(pages, sorted_pages)| match pages == sorted_pages {
            true => Some(pages[pages.len() / 2]),
            false => None,
        })
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Problem) -> isize {
    let new_input = input.clone();
    std::iter::zip(new_input.instructions, new_input.sorted_instructions)
        .filter_map(|(pages, sorted_pages)| match pages != sorted_pages {
            true => Some(sorted_pages[pages.len() / 2]),
            false => None,
        })
        .sum()
}
