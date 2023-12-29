use std::io::{stdin, Read};

use utils::rect::Rect;

fn main() {
    let input: Vec<Rect<bool>> = {
        let mut input = String::new();
        stdin().read_to_string(&mut input).unwrap();
        input
            .trim()
            .split("\n\n")
            .map(|block| {
                block
                    .split('\n')
                    .map(|line| line.chars().map(|c| c == '#').collect())
                    .collect()
            })
            .collect()
    };

    fn vert_reflection(block: &Rect<bool>, k: usize) -> Option<usize> {
        (1..block.width()).find(|&x| {
            k == (0..block.height())
                .flat_map(|y| {
                    (0..Ord::min(x, block.width() - x))
                        .filter(move |&dx| block[(x + dx, y)] != block[(x - 1 - dx, y)])
                })
                .count()
        })
    }

    fn horiz_reflection(block: &Rect<bool>, k: usize) -> Option<usize> {
        (1..block.height()).find(|&y| {
            k == (0..Ord::min(y, block.height() - y))
                .flat_map(|dy| {
                    (0..block.width())
                        .filter(move |&x| block[(x, y + dy)] != block[(x, y - 1 - dy)])
                })
                .count()
        })
    }

    let solution = |k| {
        input
            .iter()
            .map(|block| {
                vert_reflection(block, k)
                    .or_else(|| horiz_reflection(block, k).map(|n| n * 100))
                    .unwrap()
            })
            .sum::<usize>()
    };

    let part1 = solution(0);
    let part2 = solution(1);

    println!("{}", part1);
    println!("{}", part2);
}
