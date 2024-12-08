use hashbrown::HashSet;

pub struct Problem {
    obstructions: HashSet<(isize, isize)>,
    start_location: (isize, isize),
    width: isize,
    height: isize,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Problem {
    let grid: Vec<Vec<&str>> = input.lines().map(|row| row.split("").collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    let mut obstructions: HashSet<(isize, isize)> = HashSet::new();
    let mut start_location = (0, 0);
    for y_pos in 0..height {
        for x_pos in 0..width {
            let this_pos = (y_pos as isize, x_pos as isize);
            let this_entry = grid[y_pos][x_pos];
            match this_entry {
                "#" => _ = obstructions.insert(this_pos),
                "^" => start_location = this_pos,
                _ => continue,
            }
        }
    }
    Problem {
        obstructions,
        start_location,
        width: width as isize,
        height: height as isize,
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Problem) -> isize {
    let mut direction: (isize, isize) = (-1, 0);
    let mut pos: (isize, isize) = input.start_location.clone();
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    while (pos.0 >= 0) & (pos.0 < input.height) & (pos.1 >= 0) & (pos.1 < input.width) {
        visited_positions.insert(pos);
        let mut new_position = (pos.0 + direction.0, pos.1 + direction.1);
        if input.obstructions.contains(&new_position) {
            direction = (direction.1, -1 * direction.0);
            new_position = (pos.0 + direction.0, pos.1 + direction.1);
        }
        pos = new_position;
    }
    visited_positions.len() as isize
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Problem) -> isize {
    let mut direction: (isize, isize) = (-1, 0);
    let mut pos: (isize, isize) = input.start_location.clone();
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    while (pos.0 >= 0) & (pos.0 < input.height) & (pos.1 >= 0) & (pos.1 < input.width) {
        visited_positions.insert(pos);
        let mut new_position = (pos.0 + direction.0, pos.1 + direction.1);
        if input.obstructions.contains(&new_position) {
            direction = (direction.1, -1 * direction.0);
            new_position = (pos.0 + direction.0, pos.1 + direction.1);
        }
        pos = new_position;
    }

    let mut num_looping_instructions = 0;
    for position in visited_positions {
        // Setup this run
        let mut this_obstructions = input.obstructions.clone();
        this_obstructions.insert(position);
        // Run!
        let mut direction: (isize, isize) = (-1, 0);
        let mut pos: (isize, isize) = input.start_location.clone();
        let mut steps = 0;
        while (pos.0 >= 0) & (pos.0 < input.height) & (pos.1 >= 0) & (pos.1 < input.width) {
            let mut new_position = (pos.0 + direction.0, pos.1 + direction.1);
            if this_obstructions.contains(&new_position) {
                direction = (direction.1, -1 * direction.0);
                new_position = (pos.0 + direction.0, pos.1 + direction.1);
            }
            // check for loop
            if steps > 2 * (input.width * input.height) {
                num_looping_instructions += 1;
                break;
            }
            pos = new_position;
            steps += 1;
        }
    }
    num_looping_instructions
}
