use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(sequence: &[usize]) -> bool {
    let mut sum_diff_sign = 0;
    for (v1, v2) in sequence.iter().tuple_windows() {
        let diff = (v2 - v1) as i32;
        sum_diff_sign += diff.signum();
        if (diff.abs() < 1) | (diff.abs() > 3) {
            // unsafe if dangrous magnitude difference
            return false;
        }
    }
    // check if directions are consistent (sum of signs == num of diffs)
    return sum_diff_sign.abs() == (sequence.len() - 1) as i32;
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    input.iter().filter(|row| is_safe(row)).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|row| {
            if is_safe(row) {
                return 1;
            }
            for mask_ind in 0..row.len() {
                let mut masked_row = row.clone();
                masked_row.remove(mask_ind);
                if is_safe(&masked_row) {
                    return 1;
                }
            }
            return 0;
        })
        .sum()
}
