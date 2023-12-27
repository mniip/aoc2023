use std::io::stdin;

use itertools::iterate;
use utils::{
    looping_iter::Delooping,
    rect::{Rect, Transposed},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Rolling,
    Static,
}

fn main() {
    let input: Rect<Cell> = stdin()
        .lines()
        .map(|res| {
            res.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    'O' => Cell::Rolling,
                    '#' => Cell::Static,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    fn roll_north(mut board: Rect<Cell>) -> Rect<Cell> {
        for y in 0..board.height() {
            for x in 0..board.width() {
                if board[(x, y)] == Cell::Rolling {
                    board[(x, y)] = Cell::Empty;
                    let mut new_y = y;
                    while new_y > 0 && board[(x, new_y - 1)] == Cell::Empty {
                        new_y -= 1;
                    }
                    board[(x, new_y)] = Cell::Rolling;
                }
            }
        }
        board
    }

    fn rotate_cw(board: Rect<Cell>) -> Rect<Cell> {
        Transposed(board)
            .into_iter()
            .map(|mut row| {
                row.reverse();
                row
            })
            .collect()
    }

    fn load(board: &Rect<Cell>) -> usize {
        board
            .cells()
            .filter_map(|(_, y, &c)| (c == Cell::Rolling).then_some(board.height() - y))
            .sum()
    }

    let part1 = load(&roll_north(input.clone()));

    let part2 = load(
        &iterate(input.clone(), |b| rotate_cw(roll_north(b.clone())))
            .delooping()
            .nth(4 * 1000000000)
            .unwrap(),
    );

    println!("{}", part1);
    println!("{}", part2);
}
