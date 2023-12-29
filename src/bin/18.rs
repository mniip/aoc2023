use std::io::stdin;

use regex::Regex;
use utils::direction::Direction4;

fn main() {
    let move_re = Regex::new(r"^([RDLU]) (\d+) \(#([0-9a-f]+)\)$").unwrap();
    let input: Vec<(char, u32, u32)> = stdin()
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let [dir, len, color] = move_re.captures(&line).unwrap().extract().1;
            (
                dir.chars().next().unwrap(),
                str::parse(len).unwrap(),
                u32::from_str_radix(color, 16).unwrap(),
            )
        })
        .collect();

    fn solution<I: Iterator<Item = (Direction4, u32)>>(iter: I) -> u64 {
        let mut len = 0;
        let mut area: i64 = 0;
        let mut pos = (0, 0);
        for (dir, dist) in iter {
            len += dist;
            pos = dir.advance_by(pos, dist as i32);
            area += dist as i64
                * match dir {
                    Direction4::South => pos.0,
                    Direction4::North => -pos.0,
                    Direction4::West => pos.1,
                    Direction4::East => -pos.1,
                } as i64;
        }
        (area.unsigned_abs() + len as u64) / 2 + 1
    }

    let part1 = solution(input.iter().map(|&(dir, dist, _)| {
        (
            match dir {
                'R' => Direction4::East,
                'D' => Direction4::South,
                'L' => Direction4::West,
                'U' => Direction4::North,
                _ => panic!(),
            },
            dist,
        )
    }));
    let part2 = solution(input.iter().map(|(_, _, color)| {
        (
            match color & 0xF {
                0 => Direction4::East,
                1 => Direction4::South,
                2 => Direction4::West,
                3 => Direction4::North,
                _ => panic!(),
            },
            color >> 4,
        )
    }));

    println!("{}", part1);
    println!("{}", part2);
}
