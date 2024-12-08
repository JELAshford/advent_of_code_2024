use crypto::common::typenum::operator_aliases;
use itertools::repeat_n;
use itertools::Itertools;

pub struct Problem {
    target: isize,
    values: Vec<isize>,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Plus,
    Mult,
    Concat,
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

fn can_find_target(target: isize, values: Vec<isize>, operators: Vec<Operator>) -> Option<usize> {
    let num_operators = values.len() - 1;
    for operations in repeat_n(operators, num_operators).multi_cartesian_product() {
        let mut total = values[0].clone();
        for (ind, operator) in operations.iter().enumerate() {
            match operator {
                Operator::Mult => total *= values[ind + 1],
                Operator::Plus => total += values[ind + 1],
                Operator::Concat => {
                    let mut first_str = total.to_string();
                    let second_str = values[ind + 1].to_string();
                    first_str.push_str(&second_str);
                    total = first_str.parse().unwrap()
                }
            }
        }
        if total == target {
            return Some(target as usize);
        }
    }
    return None;
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<Problem>) -> usize {
    input
        .iter()
        .filter_map(|problem| {
            can_find_target(
                problem.target,
                problem.values.clone(),
                vec![Operator::Mult, Operator::Plus],
            )
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<Problem>) -> usize {
    input
        .iter()
        .filter_map(|problem| {
            can_find_target(
                problem.target,
                problem.values.clone(),
                vec![Operator::Mult, Operator::Plus, Operator::Concat],
            )
        })
        .sum()
}
