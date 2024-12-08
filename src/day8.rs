use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct StationGroup {
    positions: HashMap<u8, Vec<(isize, isize)>>,
    width: isize,
    height: isize,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> StationGroup {
    let height = input.lines().count() as isize;
    let width = input.lines().peekable().peek().unwrap().len() as isize;
    let mut positions = HashMap::<u8, Vec<(isize, isize)>>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.bytes().enumerate() {
            if val != b'.' {
                let pos = (y as isize, x as isize);
                _ = positions
                    .entry(val)
                    .and_modify(|v| v.push(pos))
                    .or_insert(vec![pos])
            }
        }
    }
    StationGroup {
        positions,
        width,
        height,
    }
}

fn generate_part1_antinodes(
    station_positions: &[(isize, isize)],
    width: isize,
    height: isize,
) -> Vec<(isize, isize)> {
    let mut antinode_positions = Vec::<(isize, isize)>::new();
    for ind1 in 0..station_positions.len() {
        let pos1 = station_positions[ind1];
        for ind2 in (ind1 + 1)..station_positions.len() {
            let pos2 = station_positions[ind2];
            let (dy, dx) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
            let a1 = (pos1.0 - dy, pos1.1 - dx);
            let a2 = (pos2.0 + dy, pos2.1 + dx);
            if (a1.0 >= 0) & (a1.0 < height) & (a1.1 >= 0) & (a1.1 < width) {
                antinode_positions.push(a1);
            }
            if (a2.0 >= 0) & (a2.0 < height) & (a2.1 >= 0) & (a2.1 < width) {
                antinode_positions.push(a2);
            }
        }
    }
    antinode_positions
}

fn generate_part2_antinodes(
    station_positions: &[(isize, isize)],
    width: isize,
    height: isize,
) -> Vec<(isize, isize)> {
    let mut antinode_positions = Vec::<(isize, isize)>::new();
    if station_positions.len() <= 1 {
        return vec![];
    }
    for ind1 in 0..station_positions.len() {
        let pos1 = station_positions[ind1];
        for ind2 in (ind1 + 1)..station_positions.len() {
            let pos2 = station_positions[ind2];

            let (dy, dx) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
            // one way ...
            let mut new_pos = pos1.clone();
            while (new_pos.0 >= 0) & (new_pos.0 < height) & (new_pos.1 >= 0) & (new_pos.1 < width) {
                antinode_positions.push(new_pos);
                new_pos = (new_pos.0 - dy, new_pos.1 - dx);
            }
            // ... or another
            let mut new_pos = pos1.clone();
            while (new_pos.0 >= 0) & (new_pos.0 < height) & (new_pos.1 >= 0) & (new_pos.1 < width) {
                antinode_positions.push(new_pos);
                new_pos = (new_pos.0 + dy, new_pos.1 + dx);
            }
        }
    }
    antinode_positions
}
#[aoc(day8, part1)]
pub fn solve_part1(input: &StationGroup) -> usize {
    input
        .positions
        .iter()
        .map(|(_id, pos)| {
            generate_part1_antinodes(pos, input.width as isize, input.height as isize)
        })
        .flatten()
        .unique()
        .count()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &StationGroup) -> usize {
    input
        .positions
        .iter()
        .map(|(_id, pos)| {
            generate_part2_antinodes(pos, input.width as isize, input.height as isize)
        })
        .flatten()
        .unique()
        .count()
}
