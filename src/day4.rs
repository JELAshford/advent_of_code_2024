use itertools::iproduct;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> isize {
    let num_rows = input.len() as isize;
    let num_cols = input[0].len() as isize;
    let mut word_counts = 0;
    for (y, x) in iproduct!(0..num_rows, 0..num_cols) {
        if input[y as usize][x as usize] == 'X' {
            for direction in iproduct!(-1..=1, -1..=1) {
                let slice: String = (0..=3)
                    .filter_map(|step| {
                        let new_y = y + direction.0 * step;
                        let new_x = x + direction.1 * step;
                        if (new_y < 0) | (new_y >= num_rows) | (new_x < 0) | (new_x >= num_cols) {
                            return None;
                        } else {
                            return Some(input[new_y as usize][new_x as usize]);
                        }
                    })
                    .collect();
                if slice == "XMAS" {
                    word_counts += 1;
                }
            }
        }
    }
    word_counts
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> isize {
    let num_rows = input.len() as isize;
    let num_cols = input[0].len() as isize;
    // Corners in order (tl, br, tr, bl)
    let corners: Vec<(isize, isize)> = vec![(-1, -1), (1, 1), (-1, 1), (1, -1)];
    let mut word_counts = 0;
    for (y, x) in iproduct!(0..num_rows, 0..num_cols) {
        if input[y as usize][x as usize] == 'A' {
            let corner_vals: String = corners
                .iter()
                .filter_map(|offset| {
                    let new_y = y + offset.0;
                    let new_x = x + offset.1;
                    if (new_y < 0) | (new_y >= num_rows) | (new_x < 0) | (new_x >= num_cols) {
                        return None;
                    } else {
                        return Some(input[new_y as usize][new_x as usize]);
                    }
                })
                .collect();
            // If none lost at the boundary
            if corner_vals.len() == 4 {
                // Check first and last pair each contain one "M" and one "S"
                if (&corner_vals[0..2] == "MS") | (&corner_vals[0..2] == "SM") {
                    if (&corner_vals[2..4] == "MS") | (&corner_vals[2..4] == "SM") {
                        word_counts += 1;
                    }
                }
            }
        }
    }
    word_counts
}
