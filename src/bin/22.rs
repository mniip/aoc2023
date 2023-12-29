use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::stdin,
    iter::repeat_with,
};

use itertools::Itertools;

fn main() {
    type Point = (u32, u32, u32);
    let blocks: Vec<(Point, Point)> = stdin()
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let (p1, p2) = line.split_once('~').unwrap();
            let (x1, p1) = p1.split_once(',').unwrap();
            let (y1, z1) = p1.split_once(',').unwrap();
            let (x2, p2) = p2.split_once(',').unwrap();
            let (y2, z2) = p2.split_once(',').unwrap();
            let x1: u32 = str::parse(x1).unwrap();
            let y1: u32 = str::parse(y1).unwrap();
            let z1: u32 = str::parse(z1).unwrap();
            let x2 = str::parse(x2).unwrap();
            let y2 = str::parse(y2).unwrap();
            let z2 = str::parse(z2).unwrap();
            (
                (Ord::min(x1, x2), Ord::min(y1, y2), Ord::min(z1, z2)),
                (
                    x1.abs_diff(x2) + 1,
                    y1.abs_diff(y2) + 1,
                    z1.abs_diff(z2) + 1,
                ),
            )
        })
        .collect();

    let (blocks, rests_on) = {
        let mut blocks = blocks.clone();
        blocks.sort_unstable_by_key(|&((_, _, z), _)| z);
        let mut height_map: HashMap<(u32, u32), (u32, usize)> = HashMap::new();
        let mut rests_on = Vec::new();
        for (i, &mut ((x, y, ref mut z), (size_x, size_y, size_z))) in blocks.iter_mut().enumerate()
        {
            let mut rests = HashSet::new();
            let mut min_z = 0;
            for x in x..x + size_x {
                for y in y..y + size_y {
                    let Some(&(z, j)) = height_map.get(&(x, y)) else {
                        continue;
                    };
                    match Ord::cmp(&z, &min_z) {
                        Ordering::Greater => {
                            min_z = z;
                            rests.clear();
                            rests.insert(j);
                        }
                        Ordering::Equal => {
                            rests.insert(j);
                        }
                        Ordering::Less => continue,
                    }
                }
            }
            *z = min_z + 1;
            for x in x..x + size_x {
                for y in y..y + size_y {
                    height_map.insert((x, y), (min_z + size_z, i));
                }
            }
            rests_on.push(rests);
        }
        (blocks, rests_on)
    };

    let part1 = {
        blocks.len()
            - rests_on
                .iter()
                .filter_map(|r| match r.iter().collect_vec()[..] {
                    [on] => Some(on),
                    _ => None,
                })
                .collect::<HashSet<_>>()
                .len()
    };

    let part2 = {
        let mut supports: Vec<_> = repeat_with(HashSet::new).take(blocks.len()).collect();
        for (i, rests) in rests_on.iter().enumerate() {
            for &j in rests {
                supports[j].insert(i);
            }
        }

        (0..blocks.len())
            .map(|seed| {
                let mut fallen = HashSet::new();
                let mut stack = Vec::new();
                stack.push(seed);
                while let Some(i) = stack.pop() {
                    if !fallen.insert(i) {
                        continue;
                    }
                    for &j in &supports[i] {
                        if rests_on[j].iter().all(|k| fallen.contains(k)) {
                            stack.push(j)
                        }
                    }
                }
                (fallen.len() - 1) as u32
            })
            .sum::<u32>()
    };

    println!("{}", part1);
    println!("{}", part2);
}
