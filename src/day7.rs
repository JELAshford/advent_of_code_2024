use hashbrown::HashSet;

pub struct Problem {
    target: isize,
    values: Vec<isize>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Problem> {
    input
        .lines()
        .map(|line| {
            let split_out: Vec<&str> = line.split(": ").collect();
            Problem {
                target: split_out[0].parse().unwrap(),
                values: split_out[1]
                    .split(" ")
                    .map(|v| v.parse::<isize>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn concat(a: isize, b: isize, base: isize) -> isize {
    assert!(base >= 2);
    a * isize::from(base).pow(b.ilog(base) + 1) + b
}

fn can_find_target(target: isize, values: Vec<isize>, use_concat: bool) -> Option<usize> {
    // from @lord_braleigh
    let mut numbers_we_can_make = HashSet::<isize>::new();
    numbers_we_can_make.insert(0);
    for new_number in values {
        let mut new_numbers_we_can_make = HashSet::<isize>::new();
        for old_number in numbers_we_can_make {
            if old_number > target {
                continue;
            }
            new_numbers_we_can_make.insert(old_number + new_number);
            new_numbers_we_can_make.insert(old_number * new_number);
            if use_concat {
                new_numbers_we_can_make.insert(concat(old_number, new_number, 10));
            }
        }
        numbers_we_can_make = new_numbers_we_can_make;
    }
    if numbers_we_can_make.contains(&target) {
        return Some(target as usize);
    } else {
        return None;
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<Problem>) -> usize {
    input
        .iter()
        .filter_map(|problem| can_find_target(problem.target, problem.values.clone(), false))
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<Problem>) -> usize {
    input
        .iter()
        .filter_map(|problem| can_find_target(problem.target, problem.values.clone(), true))
        .sum()
}
