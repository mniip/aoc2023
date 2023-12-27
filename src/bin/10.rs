use std::io::stdin;

use strum::IntoEnumIterator;
use utils::{direction::Direction4, rect::Rect};

fn pipe_dirs(c: char) -> Option<(Direction4, Direction4)> {
    match c {
        '|' => Some((Direction4::North, Direction4::South)),
        '-' => Some((Direction4::East, Direction4::West)),
        'L' => Some((Direction4::East, Direction4::North)),
        'J' => Some((Direction4::North, Direction4::West)),
        '7' => Some((Direction4::West, Direction4::South)),
        'F' => Some((Direction4::East, Direction4::South)),
        _ => None,
    }
}

struct BoundaryIter<'a> {
    field: &'a Rect<char>,
    pos: Option<(isize, isize)>,
    dir: Direction4,
}

impl<'a> BoundaryIter<'a> {
    fn new(field: &'a Rect<char>) -> Option<Self> {
        let pos = field
            .cells()
            .find_map(|(x, y, &c)| (c == 'S').then_some((x as isize, y as isize)))?;
        let dir = Direction4::iter().find(|&d| {
            field
                .get(d.advance(pos))
                .copied()
                .and_then(pipe_dirs)
                .is_some_and(|(d1, d2)| d1 == d.opposite() || d2 == d.opposite())
        })?;
        Some(BoundaryIter {
            field,
            pos: Some(pos),
            dir,
        })
    }
}

impl<'a> Iterator for BoundaryIter<'a> {
    type Item = ((isize, isize), Direction4);

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.dir;
        let pos = dir.advance(self.pos?);
        match *self.field.get(pos)? {
            'S' => {
                self.pos = None;
            }
            c => {
                self.pos = Some(pos);
                let (d1, d2) = pipe_dirs(c)?;
                self.dir = if d1 == dir.opposite() { d2 } else { d1 };
            }
        };
        Some((pos, dir))
    }
}

fn main() {
    let field: Rect<char> = stdin()
        .lines()
        .map(|res| res.unwrap().chars().collect())
        .collect();

    let part1 = BoundaryIter::new(&field).unwrap().count() / 2;
    let part2 = {
        let mut len = 0;
        let mut area = 0;
        for ((x, y), d) in BoundaryIter::new(&field).unwrap() {
            len += 1;
            area += match d {
                Direction4::East => -y,
                Direction4::North => -x,
                Direction4::West => y,
                Direction4::South => x,
            }
        }
        (area.abs() - len) / 2 + 1
    };

    println!("{}", part1);
    println!("{}", part2);
}
