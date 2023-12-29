use itertools::{chain, Itertools};
use std::{convert::identity, io::stdin};
use utils::{direction::Direction4, rect::Rect};

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    SplitterH,
    SplitterV,
    MirrorNWSE,
    MirrorNESW,
}

fn main() {
    let board: Rect<Cell> = stdin()
        .lines()
        .map(|res| {
            res.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '-' => Cell::SplitterH,
                    '|' => Cell::SplitterV,
                    '\\' => Cell::MirrorNWSE,
                    '/' => Cell::MirrorNESW,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let solution = |init_pos, init_dir| {
        fn walk_out(
            board: &Rect<Cell>,
            energized: &mut Rect<[bool; 4]>,
            pos: (isize, isize),
            dir: Direction4,
        ) {
            walk(board, energized, dir.advance(pos), dir)
        }

        fn walk(
            board: &Rect<Cell>,
            energized: &mut Rect<[bool; 4]>,
            pos: (isize, isize),
            dir: Direction4,
        ) {
            let Some(mask) = energized.get_mut(pos) else {
                return;
            };
            if mask[dir.opposite() as usize] {
                return;
            }
            mask[dir.opposite() as usize] = true;
            match board[pos] {
                Cell::Empty => walk_out(board, energized, pos, dir),
                Cell::SplitterH => match dir {
                    Direction4::East | Direction4::West => walk_out(board, energized, pos, dir),
                    Direction4::North | Direction4::South => {
                        walk_out(board, energized, pos, Direction4::East);
                        walk_out(board, energized, pos, Direction4::West);
                    }
                },
                Cell::SplitterV => match dir {
                    Direction4::North | Direction4::South => walk_out(board, energized, pos, dir),
                    Direction4::East | Direction4::West => {
                        walk_out(board, energized, pos, Direction4::North);
                        walk_out(board, energized, pos, Direction4::South);
                    }
                },
                Cell::MirrorNESW => walk_out(board, energized, pos, dir.reflect_nesw()),
                Cell::MirrorNWSE => walk_out(board, energized, pos, dir.reflect_nwse()),
            }
        }

        let mut energized: Rect<[bool; 4]> =
            board.iter().map(|r| vec![[false; 4]; r.len()]).collect();
        walk(&board, &mut energized, init_pos, init_dir);
        energized
            .cells()
            .filter(|(_, _, mask)| mask.iter().copied().any(identity))
            .count()
    };

    let part1 = solution((0, 0), Direction4::East);
    let part2 = {
        chain(
            (0..board.width())
                .cartesian_product([(Direction4::South, 0), (Direction4::North, board.height() - 1)])
                .map(|(x, (dir, y))| ((x as isize, y as isize), dir)),
            (0..board.height())
                .cartesian_product([(Direction4::East, 0), (Direction4::West, board.width() - 1)])
                .map(|(y, (dir, x))| ((x as isize, y as isize), dir)),
        )
        .map(|(pos, dir)| solution(pos, dir))
        .max()
        .unwrap()
    };

    println!("{}", part1);
    println!("{}", part2);
}
