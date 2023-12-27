use std::{collections::HashSet, io::stdin};

use strum::IntoEnumIterator;
use utils::{direction::Direction4, rect::Rect};

fn main() {
    let board: Rect<char> = stdin()
        .lines()
        .map(|res| res.unwrap().chars().collect())
        .collect();

    fn get_wrapping(board: &Rect<char>, pos: (isize, isize)) -> char {
        let pos = (
            pos.0.rem_euclid(board.width() as isize),
            pos.1.rem_euclid(board.height() as isize),
        );
        board[pos]
    }

    let start_pos = board
        .cells()
        .filter_map(|(x, y, &c)| (c == 'S').then_some((x as isize, y as isize)))
        .next()
        .unwrap();

    fn next_steps<'a, I: Iterator<Item = (isize, isize)> + 'a>(
        board: &'a Rect<char>,
        cells: I,
    ) -> impl Iterator<Item = (isize, isize)> + 'a {
        cells.flat_map(move |pos| {
            Direction4::iter().filter_map(move |dir| {
                let pos = dir.advance(pos);
                board.get(pos).is_some_and(|&c| c != '#').then_some(pos)
            })
        })
    }

    fn next_steps_wrapping<'a, I: Iterator<Item = (isize, isize)> + 'a>(
        board: &'a Rect<char>,
        cells: I,
    ) -> impl Iterator<Item = (isize, isize)> + 'a {
        cells.flat_map(move |pos| {
            Direction4::iter().filter_map(move |dir| {
                let pos = dir.advance(pos);
                (get_wrapping(board, pos) != '#').then_some(pos)
            })
        })
    }

    let part1 = {
        let mut cells: HashSet<(isize, isize)> = [start_pos].into();
        for _ in 0..64 {
            cells = next_steps(&board, cells.into_iter()).collect();
        }
        cells.len()
    };

    let part2 = {
        let mut perimeter: HashSet<(isize, isize)> = [start_pos].into();
        let mut prev = HashSet::new();
        let mut firsts = Vec::new();
        let mut period = 0;
        let period = loop {
            let result = next_steps_wrapping(&board, perimeter.iter().copied())
                .filter(|pos| !prev.contains(pos))
                .collect();
            prev = perimeter;
            perimeter = result;

            firsts.push(perimeter.len());
            period += 1;

            let num = perimeter
                .iter()
                .copied()
                .filter(|&pos| get_wrapping(&board, pos) == 'S')
                .count();
            if num > 0 {
                assert_eq!(num, 4);
                break period;
            }
        };
        let mut seconds = Vec::new();
        for _ in 0..period {
            let result = next_steps_wrapping(&board, perimeter.iter().copied())
                .filter(|pos| !prev.contains(pos))
                .collect();
            prev = perimeter;
            perimeter = result;

            seconds.push(perimeter.len());
        }
        (0..=26501365)
            .rev()
            .step_by(2)
            .map(|n| {
                if n == 0 {
                    1
                } else {
                    let rem = (n - 1) % period;
                    let quot = (n - 1) / period;
                    (quot * (seconds[rem] - firsts[rem]) + firsts[rem]) as u64
                }
            })
            .sum::<u64>()
    };

    println!("{}", part1);
    println!("{}", part2);
}
