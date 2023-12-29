use std::{cmp::Reverse, io::stdin, mem};

use priority_queue::PriorityQueue;
use utils::{direction::Direction4, rect::Rect};

fn main() {
    let field: Rect<u8> = stdin()
        .lines()
        .map(|res| {
            res.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let part1 = {
        let mut seen = field
            .iter()
            .map(|r| vec![[[false; 4]; 3]; r.len()])
            .collect::<Rect<_>>();
        let mut queue = PriorityQueue::new();
        for dir in [Direction4::East, Direction4::South] {
            queue.push_increase((dir.advance((0, 0)), 0u8, dir), Reverse(0));
        }
        loop {
            let Some(((pos, consecutive, dir), Reverse(len))) = queue.pop() else {
                panic!()
            };
            let Some(&step) = field.get(pos) else {
                continue;
            };
            let len = len + step as u32;
            if mem::replace(&mut seen[pos][consecutive as usize][dir as usize], true) {
                continue;
            }
            if pos == (field.width() as isize - 1, field.height() as isize - 1) {
                break len;
            }
            for dir in [dir.rotate_ccw(), dir.rotate_cw()] {
                queue.push_increase((dir.advance(pos), 0, dir), Reverse(len));
            }
            if consecutive < 2 {
                queue.push_increase((dir.advance(pos), consecutive + 1, dir), Reverse(len));
            }
        }
    };

    let part2 = {
        let mut seen = field
            .iter()
            .map(|r| vec![[[false; 4]; 10]; r.len()])
            .collect::<Rect<_>>();
        let mut queue = PriorityQueue::new();
        for dir in [Direction4::East, Direction4::South] {
            queue.push_increase((dir.advance((0, 0)), 0u8, dir), Reverse(0));
        }
        loop {
            let Some(((pos, consecutive, dir), Reverse(len))) = queue.pop() else {
                panic!()
            };
            let Some(&step) = field.get(pos) else {
                continue;
            };
            let len = len + step as u32;
            if mem::replace(&mut seen[pos][consecutive as usize][dir as usize], true) {
                continue;
            }
            if consecutive >= 3 {
                if pos == (field.width() as isize - 1, field.height() as isize - 1) {
                    break len;
                }
                for dir in [dir.rotate_ccw(), dir.rotate_cw()] {
                    queue.push_increase((dir.advance(pos), 0, dir), Reverse(len));
                }
            }
            if consecutive < 9 {
                queue.push_increase((dir.advance(pos), consecutive + 1, dir), Reverse(len));
            }
        }
    };

    println!("{}", part1);
    println!("{}", part2);
}
