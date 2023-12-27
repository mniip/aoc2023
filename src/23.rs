use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

use strum::IntoEnumIterator;
use utils::{direction::Direction4, rect::Rect};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Only(Direction4),
}

fn main() {
    let field: Rect<Cell> = stdin()
        .lines()
        .map(|res| {
            res.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    '>' => Cell::Only(Direction4::East),
                    'v' => Cell::Only(Direction4::South),
                    '<' => Cell::Only(Direction4::West),
                    '^' => Cell::Only(Direction4::North),
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    fn solution(field: &Rect<Cell>) -> usize {
        let enterable = |pos| match field.get(pos) {
            Some(Cell::Wall) => false,
            None => false,
            _ => true,
        };

        let junctions: Vec<(isize, isize)> = field
            .cells()
            .filter_map(|(x, y, c)| match c {
                Cell::Wall => None,
                _ => (Direction4::iter()
                    .filter(|&dir| enterable(dir.advance((x as isize, y as isize))))
                    .count()
                    != 2)
                    .then_some((x as isize, y as isize)),
            })
            .collect();

        let graph: Vec<HashMap<usize, usize>> = junctions
            .iter()
            .copied()
            .map(|pos| {
                Direction4::iter()
                    .filter_map(|mut dir| {
                        let mut pos = dir.advance(pos);
                        let mut distance = 1;
                        loop {
                            if let Some(i) = junctions.iter().copied().position(|p| p == pos) {
                                break Some((i, distance));
                            }
                            match field.get(pos) {
                                None => break None,
                                Some(Cell::Wall) => break None,
                                Some(&Cell::Only(d)) => {
                                    if d != dir {
                                        break None;
                                    }
                                }
                                Some(Cell::Empty) => match Direction4::iter()
                                    .filter(|&d| d != dir.opposite() && enterable(d.advance(pos)))
                                    .next()
                                {
                                    Some(d) => dir = d,
                                    None => break None,
                                },
                            }
                            pos = dir.advance(pos);
                            distance += 1;
                        }
                    })
                    .collect()
            })
            .collect();

        let start = junctions.iter().position(|&(_, y)| y == 0).unwrap();
        let finish = junctions
            .iter()
            .position(|&(_, y)| y == (field.height() - 1) as isize)
            .unwrap();

        let mut journeys = VecDeque::new();
        journeys.push_back((start, 0, HashSet::new()));
        let mut max_len = 0;
        while let Some((i, len, mut seen)) = journeys.pop_front() {
            if i == finish {
                max_len = Ord::max(max_len, len);
                continue;
            }
            seen.insert(i);
            for (&j, &distance) in &graph[i] {
                if !seen.contains(&j) {
                    journeys.push_back((j, len + distance, seen.clone()))
                }
            }
        }
        max_len
    }

    let part1 = solution(&field);
    let part2 = {
        let mut field = field;
        for (_, _, cell) in field.cells_mut() {
            match cell {
                Cell::Only(_) => *cell = Cell::Empty,
                _ => (),
            }
        }
        solution(&field)
    };

    println!("{}", part1);
    println!("{}", part2);
}
