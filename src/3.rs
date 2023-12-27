use std::{io::stdin, ops::RangeInclusive};

use itertools::Itertools;
use strum::IntoEnumIterator;
use utils::{direction::Direction8, rect::Rect};

fn main() {
    let field: Rect<char> = stdin()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    fn neighbors(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        Direction8::iter().map(move |d| d.advance(pos))
    }

    type NumberRange = (RangeInclusive<usize>, usize);
    let find_number_range = |(x, y)| -> Option<NumberRange> {
        if !field.get((x, y)).is_some_and(char::is_ascii_digit) {
            return None;
        }
        let mut minx = x;
        while minx > 0 && field.get((minx - 1, y)).is_some_and(char::is_ascii_digit) {
            minx -= 1
        }
        let mut maxx = x;
        while field.get((maxx + 1, y)).is_some_and(char::is_ascii_digit) {
            maxx += 1
        }
        Some((minx..=maxx, y))
    };
    let read_number_range = |(range, y): NumberRange| {
        field.iter().nth(y).unwrap()[range]
            .iter()
            .copied()
            .fold(0, |acc, d| acc * 10 + (d.to_digit(10).unwrap()))
    };

    let part1 = field
        .cells()
        .filter_map(|(x, y, &c)| (c.is_ascii_punctuation() && c != '.').then_some((x, y)))
        .flat_map(neighbors)
        .filter_map(find_number_range)
        .unique()
        .map(read_number_range)
        .sum::<u32>();
    let part2 = field
        .cells()
        .filter_map(|(x, y, &c)| (c == '*').then_some((x, y)))
        .filter_map(|p| {
            let [p1, p2] = neighbors(p)
                .filter_map(find_number_range)
                .unique()
                .collect::<Vec<_>>()
                .try_into()
                .ok()?;
            Some(read_number_range(p1) * read_number_range(p2))
        })
        .sum::<u32>();
    println!("{}", part1);
    println!("{}", part2);
}
