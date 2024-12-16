#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|v| v.to_digit(10).unwrap() as usize)
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let mut output_index = 0;
    let mut fwd_index = 0;
    let mut rev_index = input.len() - 1;
    let mut accumulator = 0;
    let mut input_copy = input.clone();
    while (fwd_index < input_copy.len()) & (rev_index > 0) {
        // if we're in some file data, calculate and add up the accumlator
        if fwd_index % 2 == 0 {
            let file_num = fwd_index / 2;
            if input_copy[fwd_index] > 0 {
                accumulator += output_index * file_num;
                input_copy[fwd_index] -= 1;
                output_index += 1;
            } else {
                fwd_index += 1;
            }
        // otherwise, fill space with the file in the reverse indexes
        } else {
            if input_copy[fwd_index] > 0 {
                if input_copy[rev_index] > 0 {
                    let file_num = rev_index / 2;
                    accumulator += output_index * file_num;
                    input_copy[rev_index] -= 1;
                    input_copy[fwd_index] -= 1;
                    output_index += 1;
                } else {
                    rev_index -= 2;
                }
            } else {
                fwd_index += 1;
            }
        }
    }
    accumulator
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    todo!()
}
