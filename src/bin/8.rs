use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    mem::swap,
};

use itertools::{iterate, Itertools};
use regex::Regex;
use utils::{
    looping_iter::Delooping,
    numbers::{bezout_identity, chinese_remainder},
};

fn main() {
    let (directions, graph) = {
        let line_re = Regex::new(r"^(\S+) = \((\S+), (\S+)\)$").unwrap();
        let mut it = stdin().lines();
        let directions = it
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => false,
                'R' => true,
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        it.next().unwrap().unwrap();
        let graph = it
            .map(|res| {
                let line = res.unwrap();
                let (_, [from, to1, to2]) = line_re.captures(&line).unwrap().extract();
                (String::from(from), [String::from(to1), String::from(to2)])
            })
            .collect::<HashMap<String, [String; 2]>>();
        (directions, graph)
    };

    let part1 = {
        let mut node = "AAA";
        let mut count = 0;
        while node != "ZZZ" {
            node = &graph[node][directions[count % directions.len()] as usize];
            count += 1;
        }
        count
    };

    let part2 = {
        struct LoopStructure {
            init_len: u64,
            init: HashSet<u128>,
            cycle_len: u128,
            cycle: HashSet<u128>,
        }

        fn is_fin_node(node: &str) -> bool {
            node.ends_with('Z')
        }

        let structure = graph
            .keys()
            .filter(|&node| node.ends_with('A'))
            .map(|node| {
                let (init, cycle) = iterate((0, node), |&(count, node)| {
                    (
                        (count + 1) % directions.len(),
                        &graph[node][directions[count] as usize],
                    )
                })
                .delooping()
                .loop_structure();
                let pops = |vec: Vec<(_, &String)>| {
                    vec.into_iter()
                        .enumerate()
                        .filter_map(|(i, (_, node))| is_fin_node(node).then_some(i as u128))
                        .collect()
                };
                LoopStructure {
                    init_len: init.len() as u64,
                    init: pops(init),
                    cycle_len: cycle.len() as u128,
                    cycle: pops(cycle),
                }
            })
            .fold(
                LoopStructure {
                    init_len: 0,
                    init: HashSet::new(),
                    cycle_len: 1,
                    cycle: [0].into(),
                },
                |mut a, mut b| {
                    if a.init_len < b.init_len {
                        swap(&mut a, &mut b);
                    }
                    a.init.retain(|&i| {
                        if i < b.init_len as u128 {
                            b.init.contains(&i)
                        } else {
                            b.cycle.contains(&((i - b.init_len as u128) % b.cycle_len))
                        }
                    });
                    let gcd = bezout_identity(a.cycle_len, b.cycle_len);
                    LoopStructure {
                        init_len: a.init_len,
                        init: a.init,
                        cycle_len: a.cycle_len * gcd.f_b,
                        cycle: a
                            .cycle
                            .into_iter()
                            .cartesian_product(b.cycle.iter())
                            .map(|(a_mod, &b_mod)| {
                                chinese_remainder(
                                    gcd,
                                    a_mod,
                                    (b_mod + b.cycle_len
                                        - (a.init_len - b.init_len) as u128 % b.cycle_len)
                                        % b.cycle_len,
                                )
                            })
                            .collect(),
                    }
                },
            );

        structure
            .init
            .into_iter()
            .min()
            .unwrap_or(structure.init_len as u128 + structure.cycle.into_iter().min().unwrap())
    };

    println!("{}", part1);
    println!("{}", part2);
}
