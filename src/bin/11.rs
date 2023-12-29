use std::{convert::identity, io::stdin, mem::swap};

use utils::rect::{Rect, Transposed};

fn main() {
    let field: Rect<bool> = stdin()
        .lines()
        .map(|res| res.unwrap().chars().map(|c| c == '#').collect())
        .collect();

    let galaxies: Vec<(usize, usize)> = field
        .cells()
        .filter_map(|(x, y, b)| b.then_some((x, y)))
        .collect();
    let blank_rows: Vec<usize> = field
        .iter()
        .map(|r| !r.iter().copied().any(identity))
        .enumerate()
        .filter_map(|(i, b)| b.then_some(i))
        .collect();
    let blank_cols: Vec<usize> = Transposed(&field)
        .iter()
        .map(|r| !r.copied().any(identity))
        .enumerate()
        .filter_map(|(i, b)| b.then_some(i))
        .collect();

    let solution = |factor: usize| {
        galaxies
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(i, g1)| galaxies[i + 1..].iter().copied().map(move |g2| (g1, g2)))
            .map(|((mut x1, mut y1), (mut x2, mut y2))| {
                if x1 > x2 {
                    swap(&mut x1, &mut x2)
                }
                if y1 > y2 {
                    swap(&mut y1, &mut y2)
                }
                (x2 - x1)
                    + (factor - 1)
                        * blank_cols
                            .iter()
                            .copied()
                            .skip_while(|&x| x < x1)
                            .take_while(|&x| x < x2)
                            .count()
                    + (y2 - y1)
                    + (factor - 1)
                        * blank_rows
                            .iter()
                            .copied()
                            .skip_while(|&y| y < y1)
                            .take_while(|&y| y < y2)
                            .count()
            })
            .sum::<usize>()
    };

    let part1 = solution(2);
    let part2 = solution(1000000);
    println!("{}", part1);
    println!("{}", part2);
}
