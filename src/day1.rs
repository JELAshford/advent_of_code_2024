use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<isize>, Vec<isize>) {
    let location_ids: Vec<&str> = input.split("\n").collect();
    let mut first_ids: Vec<isize> = Vec::with_capacity(location_ids.len());
    let mut second_ids: Vec<isize> = Vec::with_capacity(location_ids.len());
    for item in location_ids.iter() {
        let values: Vec<isize> = item.split("   ").map(|val| val.parse().unwrap()).collect();
        first_ids.push(values[0]);
        second_ids.push(values[1]);
    }
    first_ids.sort();
    second_ids.sort();
    (first_ids, second_ids)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<isize>, Vec<isize>)) -> isize {
    let (first_ids, second_ids) = input;
    std::iter::zip(first_ids, second_ids)
        .map(|(v1, v2)| (v2 - v1).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<isize>, Vec<isize>)) -> isize {
    let (first_ids, second_ids) = input;
    let mut counts: HashMap<&isize, [isize; 2]> = HashMap::new();
    for (v1, v2) in std::iter::zip(first_ids, second_ids) {
        counts.entry(v1).and_modify(|v| v[0] += 1).or_insert([1, 0]);
        counts.entry(v2).and_modify(|v| v[1] += 1).or_insert([0, 1]);
    }
    counts.iter().map(|(key, val)| *key * val[0] * val[1]).sum()
}
