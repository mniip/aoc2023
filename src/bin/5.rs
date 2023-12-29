use std::{
    cmp::{max, min},
    io::{stdin, Read},
    ops::Range,
};

struct MapEntry {
    source: Range<i64>,
    offset: i64,
}

fn main() {
    let input = {
        let mut input = String::new();
        stdin().read_to_string(&mut input).unwrap();
        input
    };
    let (seeds, maps) = {
        let mut it = input.split("\n\n");
        let seeds = it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(str::parse)
            .collect::<Result<Vec<i64>, _>>()
            .unwrap();
        let maps: Vec<Vec<MapEntry>> = it
            .map(|m| {
                m.lines()
                    .skip(1)
                    .map(|l| {
                        let [dest, src, len] = l
                            .split_ascii_whitespace()
                            .map(str::parse)
                            .collect::<Result<Vec<i64>, _>>()
                            .unwrap()
                            .try_into()
                            .unwrap();
                        MapEntry {
                            source: src..src + len,
                            offset: dest - src,
                        }
                    })
                    .collect()
            })
            .collect();
        (seeds, maps)
    };

    let part1 = seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |seed, m| {
                for e in m {
                    if e.source.contains(&seed) {
                        return seed + e.offset;
                    }
                }
                seed
            })
        })
        .min()
        .unwrap();

    let part2 = maps
        .iter()
        .fold(
            seeds
                .chunks_exact(2)
                .map(|chunk| match chunk {
                    &[start, len] => start..start + len,
                    _ => panic!(""),
                })
                .collect::<Vec<_>>(),
            |ranges, map| {
                let mut mapped = Vec::new();
                let mut input = ranges;
                for e in map {
                    let mut output = Vec::new();
                    for range in input {
                        if range.start < e.source.end && range.end > e.source.start {
                            mapped.push(
                                max(range.start, e.source.start) + e.offset
                                    ..min(range.end, e.source.end) + e.offset,
                            )
                        }
                        if range.start < e.source.start {
                            output.push(range.start..min(range.end, e.source.start))
                        }
                        if range.end > e.source.end {
                            output.push(max(range.start, e.source.end)..range.end)
                        }
                    }
                    input = output
                }
                mapped.extend(input);
                mapped
            },
        )
        .into_iter()
        .map(|r| r.start)
        .min()
        .unwrap();

    println!("{}", part1);
    println!("{}", part2);
}
