// From @AxlLind
use itertools::Itertools;

fn walk(
    m: &[Vec<u8>],
    mut r: usize,
    mut c: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; m[0].len()]; m.len()];
    let mut d = 0;
    loop {
        if seen[r][c][d] {
            return None;
        }
        seen[r][c][d] = true;
        let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][d];
        let (rr, cc) = (r + dr as usize, c + dc as usize);
        if !(0..m.len()).contains(&rr) || !(0..m[0].len()).contains(&cc) {
            if !return_squares {
                return Some(Vec::new());
            }
            let visited = (0..m.len())
                .cartesian_product(0..m[0].len())
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();
            return Some(visited);
        }
        if m[rr][cc] == b'#' {
            d = (d + 1) % 4;
        } else {
            (r, c) = (rr, cc);
        }
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let m = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let (sr, sc) = (0..m.len())
        .cartesian_product(0..m[0].len())
        .find(|&(r, c)| m[r][c] == b'^')
        .unwrap();
    (m, (sr, sc))
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &(Vec<Vec<u8>>, (usize, usize))) -> usize {
    let (m, (sr, sc)) = input;
    let p1: Vec<(usize, usize)> = walk(m, *sr, *sc, true).unwrap();
    p1.len()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &(Vec<Vec<u8>>, (usize, usize))) -> usize {
    let (m, (sr, sc)) = input;
    let mut m = m.clone();
    let p1: Vec<(usize, usize)> = walk(&m, *sr, *sc, true).unwrap();
    p1.iter()
        .filter(|&&(r, c)| {
            m[r][c] = b'#';
            let ok = walk(&m, *sr, *sc, false).is_none();
            m[r][c] = b'.';
            ok
        })
        .count()
}
