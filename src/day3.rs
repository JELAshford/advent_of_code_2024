pub enum Command {
    Mult(isize, isize),
    Do,
    Dont,
}
type Program = Vec<Command>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Program {
    let mut program: Program = vec![];
    for start_index in 0..(input.len() - 5) {
        let command_prefix = &input[start_index..(start_index + 3)];
        if command_prefix == "mul" {
            let start_offset = (start_index + 4).min(input.len() - 1);
            let end_offset = (start_index + 12).min(input.len() - 1);
            let possible_args = &input[start_offset..end_offset];
            // Attempt to parse args
            let num_segments: Vec<&str> = possible_args.split(",").collect();
            let first_val = match num_segments[0].parse::<isize>() {
                Ok(val) => val,
                Err(_error) => continue,
            };
            let second_val_str = num_segments[1].split(")").next().unwrap();
            let second_val = match second_val_str.parse::<isize>() {
                Ok(val) => val,
                Err(_error) => continue,
            };
            program.push(Command::Mult(first_val, second_val))
        } else if command_prefix == "do(" {
            let poss_do_suffix = &input[(start_index + 3)..(start_index + 4)];
            if poss_do_suffix == ")" {
                program.push(Command::Do)
            }
        } else if command_prefix == "don" {
            let poss_dont_suffix = &input[(start_index + 3)..(start_index + 7)];
            if poss_dont_suffix == "'t()" {
                program.push(Command::Dont)
            }
        }
    }
    return program;
}

#[aoc(day3, part1)]
pub fn solve_part1(program: &Program) -> isize {
    program
        .iter()
        .filter_map(|command| match command {
            Command::Do | Command::Dont => None,
            Command::Mult(v1, v2) => Some(v1 * v2),
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(program: &Program) -> isize {
    let mut do_mult = true;
    let mut total = 0;
    for command in program {
        match command {
            Command::Do => do_mult = true,
            Command::Dont => do_mult = false,
            Command::Mult(v1, v2) => {
                if do_mult {
                    total += v1 * v2;
                }
            }
        }
    }
    return total;
}
