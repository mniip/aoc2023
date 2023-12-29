use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::identity,
    io::stdin,
    mem,
};

use itertools::Itertools;
use utils::numbers::lcm;

#[derive(PartialEq, Eq, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Dummy,
}

#[derive(Debug)]
struct Module {
    kind: ModuleType,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

#[derive(Debug)]
enum ModuleState {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(Vec<bool>),
    Dummy,
}

fn main() {
    let (modules, _names, indices) = {
        let mut names = Vec::new();
        let mut modules: Vec<Result<Module, Vec<usize>>> = Vec::new();
        let mut indices = HashMap::new();
        let mut resolve =
            |modules: &mut Vec<_>, names: &mut Vec<_>, name: &str| match indices.get(name) {
                None => {
                    let index = modules.len();
                    indices.insert(String::from(name), index);
                    names.push(String::from(name));
                    modules.push(Err(Vec::new()));
                    index
                }
                Some(&index) => index,
            };
        for res in stdin().lines() {
            let line = res.unwrap();
            let (name, out) = line.split_once(" -> ").unwrap();
            let (name, kind) = match name {
                "broadcaster" => (name, ModuleType::Broadcaster),
                _ if name.starts_with('%') => (&name[1..], ModuleType::FlipFlop),
                _ if name.starts_with('&') => (&name[1..], ModuleType::Conjunction),
                _ => panic!(),
            };
            let index = resolve(&mut modules, &mut names, name);
            let mut outputs = Vec::new();
            for output in out.split(", ") {
                let output = resolve(&mut modules, &mut names, output);
                outputs.push(output);
                match &mut modules[output] {
                    Ok(module) => module.inputs.push(index),
                    Err(inputs) => inputs.push(index),
                }
            }
            match &mut modules[index] {
                Ok(_) => panic!(),
                Err(inputs) => {
                    modules[index] = Ok(Module {
                        kind,
                        inputs: mem::take(inputs),
                        outputs,
                    })
                }
            }
        }
        (
            modules
                .into_iter()
                .map(|res| match res {
                    Ok(module) => module,
                    Err(inputs) => Module {
                        kind: ModuleType::Dummy,
                        inputs,
                        outputs: Vec::new(),
                    },
                })
                .collect::<Vec<_>>(),
            names,
            indices,
        )
    };

    fn init_state(module: &Module) -> ModuleState {
        match module.kind {
            ModuleType::Broadcaster => ModuleState::Broadcaster,
            ModuleType::FlipFlop => ModuleState::FlipFlop(false),
            ModuleType::Conjunction => ModuleState::Conjunction(vec![false; module.inputs.len()]),
            ModuleType::Dummy => ModuleState::Dummy,
        }
    }

    fn handle_pulse(
        modules: &[Module],
        state: &mut [ModuleState],
        pulses: &mut VecDeque<(bool, usize, usize)>,
        high: bool,
        from: usize,
        current: usize,
    ) {
        match &mut state[current] {
            ModuleState::Broadcaster => {
                for &to in &modules[current].outputs {
                    pulses.push_back((high, current, to));
                }
            }
            ModuleState::FlipFlop(state) => {
                if !high {
                    *state = !*state;
                    for &to in &modules[current].outputs {
                        pulses.push_back((*state, current, to));
                    }
                }
            }
            ModuleState::Conjunction(conj) => {
                let index = modules[current]
                    .inputs
                    .iter()
                    .position(|&input| input == from)
                    .unwrap();
                conj[index] = high;
                let output = !conj.iter().copied().all(identity);
                for &to in &modules[current].outputs {
                    pulses.push_back((output, current, to));
                }
            }
            ModuleState::Dummy => (),
        }
    }

    let broadcaster = indices["broadcaster"];

    let part1 = {
        let mut state = modules.iter().map(init_state).collect_vec();
        let mut high_count: usize = 0;
        let mut low_count: usize = 0;
        let mut pulses = VecDeque::new();
        for _ in 0..1000 {
            pulses.push_back((false, broadcaster, broadcaster));
            while let Some((high, from, current)) = pulses.pop_front() {
                if high {
                    high_count += 1;
                } else {
                    low_count += 1;
                }
                handle_pulse(&modules, &mut state, &mut pulses, high, from, current)
            }
        }
        high_count * low_count
    };

    let part2 = {
        let rx = &modules[indices["rx"]];
        assert_eq!(rx.kind, ModuleType::Dummy);
        let conj = match rx.inputs[..] {
            [input] => &modules[input],
            _ => panic!(),
        };
        assert_eq!(conj.kind, ModuleType::Conjunction);
        conj.inputs
            .iter()
            .copied()
            .map(|inverter_idx| {
                let inverter = &modules[inverter_idx];
                assert_eq!(inverter.kind, ModuleType::Conjunction);
                let matcher = match inverter.inputs[..] {
                    [input] => &modules[input],
                    _ => panic!(),
                };
                assert_eq!(matcher.kind, ModuleType::Conjunction);
                let bits: HashSet<usize> = matcher
                    .inputs
                    .iter()
                    .copied()
                    .chain(
                        matcher
                            .outputs
                            .iter()
                            .copied()
                            .filter(|&bit| bit != inverter_idx),
                    )
                    .collect();
                let bits: Vec<usize> = {
                    let mut seq = VecDeque::new();
                    let center = bits.iter().copied().next().unwrap();
                    assert_eq!(modules[center].kind, ModuleType::FlipFlop);
                    seq.push_back(center);
                    let mut bit = center;
                    while let Some(&b) = modules[bit].outputs.iter().find(|&i| bits.contains(i)) {
                        assert_eq!(modules[b].kind, ModuleType::FlipFlop);
                        seq.push_back(b);
                        bit = b;
                    }
                    let mut bit = center;
                    while let Some(&b) = modules[bit].inputs.iter().find(|&i| bits.contains(i)) {
                        assert_eq!(modules[b].kind, ModuleType::FlipFlop);
                        seq.push_front(b);
                        bit = b;
                    }
                    assert_eq!(seq.len(), bits.len());
                    seq.into()
                };
                let test_mask = bits
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(i, bit)| matcher.inputs.contains(&bit).then_some(1 << i))
                    .sum::<u64>();
                let add_mask = bits
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(i, bit)| matcher.outputs.contains(&bit).then_some(1 << i))
                    .sum::<u64>();
                assert_eq!(test_mask + add_mask, 1 << bits.len());
                test_mask
            })
            .fold(1, lcm)
    };

    println!("{}", part1);
    println!("{}", part2);
}
