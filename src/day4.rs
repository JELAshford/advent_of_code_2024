use itertools::iproduct;

pub struct Grid {
    values: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid {
    let vals: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid {
        values: vals.clone(),
        width: vals[0].len() as isize,
        height: vals.len() as isize,
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Grid) -> isize {
    let mut word_counts = 0;
    for (y, x) in iproduct!(0..input.height, 0..input.width) {
        if input.values[y as usize][x as usize] == 'X' {
            for direction in iproduct!(-1..=1, -1..=1) {
                if direction != (0, 0) {
                    let slice: String = (0..=3)
                        .filter_map(|step| {
                            let dy = y as isize + direction.0 * step;
                            let dx = y as isize + direction.0 * step;
                            if (dy < 0) | (dy >= input.height) | (dx < 0) | (dx >= input.width) {
                                return None;
                            } else {
                                return Some(input.values[dy as usize][dx as usize]);
                            }
                        })
                        .collect();
                    if slice == "XMAS" {
                        word_counts += 1;
                    }
                }
            }
        }
    }
    word_counts
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Grid) -> isize {
    // Corners in order (tl, br, tr, bl)
    let corners: Vec<(isize, isize)> = vec![(-1, -1), (1, 1), (-1, 1), (1, -1)];
    let mut word_counts = 0;
    for (y, x) in iproduct!(1..input.height - 1, 1..input.width - 1) {
        if input.values[y as usize][x as usize] == 'A' {
            let corner_vals: String = corners
                .iter()
                .map(|offset| input.values[(y + offset.0) as usize][(x + offset.1) as usize])
                .collect();
            if (&corner_vals[0..2] == "MS") | (&corner_vals[0..2] == "SM") {
                if (&corner_vals[2..4] == "MS") | (&corner_vals[2..4] == "SM") {
                    word_counts += 1;
                }
            }
        }
    }
    word_counts
}
