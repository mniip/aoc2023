use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    io::{stdin, Read},
    str::FromStr,
};

use regex::Regex;

struct Step {
    property: usize,
    greater: bool,
    constant: u32,
    action: Action,
}

#[derive(Clone)]
enum Action {
    Accept,
    Reject,
    Delegate(String),
}

impl FromStr for Action {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::Delegate(String::from(s)),
        })
    }
}

fn main() {
    let workflow_re = Regex::new(r"^(\w+)\{(.*),(\w+)\}$").unwrap();
    let step_re = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
    let part_re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let (workflows, parts) = {
        let mut input = String::new();
        stdin().read_to_string(&mut input).unwrap();
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        let workflows: HashMap<String, (Vec<Step>, Action)> = workflows
            .lines()
            .map(|workflow| {
                let [name, steps, fallback] = workflow_re.captures(workflow).unwrap().extract().1;
                let steps = steps
                    .split(',')
                    .map(|step| {
                        let [property, op, num, action] =
                            step_re.captures(step).unwrap().extract().1;
                        let property = match property.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => panic!(),
                        };
                        let greater = match op.chars().next().unwrap() {
                            '<' => false,
                            '>' => true,
                            _ => panic!(),
                        };
                        Step {
                            property,
                            greater,
                            constant: str::parse(num).unwrap(),
                            action: str::parse(action).unwrap(),
                        }
                    })
                    .collect();
                (String::from(name), (steps, str::parse(fallback).unwrap()))
            })
            .collect();
        let parts: Vec<[u32; 4]> = parts
            .lines()
            .map(|part| {
                part_re
                    .captures(part)
                    .unwrap()
                    .extract()
                    .1
                    .map(|n| str::parse(n).unwrap())
            })
            .collect();
        (workflows, parts)
    };

    let part1 = parts
        .iter()
        .filter(|&part| {
            let mut action = Action::Delegate(String::from("in"));
            'filter: loop {
                match action {
                    Action::Accept => break true,
                    Action::Reject => break false,
                    Action::Delegate(ref workflow) => {
                        let (steps, fallback) = &workflows[workflow];
                        for step in steps {
                            if if step.greater {
                                part[step.property] > step.constant
                            } else {
                                part[step.property] < step.constant
                            } {
                                action = step.action.clone();
                                continue 'filter;
                            }
                        }
                        action = fallback.clone();
                    }
                }
            }
        })
        .flatten()
        .sum::<u32>();

    let part2 = {
        let mut accepted = 0;
        let mut queue = VecDeque::new();
        fn volume(min: [u32; 4], max: [u32; 4]) -> u64 {
            min.into_iter()
                .zip(max)
                .map(|(min, max)| (max - min + 1) as u64)
                .product()
        }
        let mut act = |queue: &mut VecDeque<_>, action: &Action, min, max| match action {
            Action::Accept => accepted += volume(min, max),
            Action::Reject => (),
            Action::Delegate(workflow) => queue.push_back((min, max, workflow.clone())),
        };
        queue.push_back(([1, 1, 1, 1], [4000, 4000, 4000, 4000], String::from("in")));
        'queue: while let Some((mut min, mut max, workflow)) = queue.pop_front() {
            let (steps, fallback) = &workflows[&workflow];
            for step in steps {
                if step.greater {
                    if max[step.property] > step.constant {
                        let mut min = min;
                        min[step.property] = step.constant + 1;
                        act(&mut queue, &step.action, min, max)
                    }
                    if min[step.property] <= step.constant {
                        max[step.property] = step.constant;
                    } else {
                        continue 'queue;
                    }
                } else {
                    if min[step.property] < step.constant {
                        let mut max = max;
                        max[step.property] = step.constant - 1;
                        act(&mut queue, &step.action, min, max)
                    }
                    if max[step.property] >= step.constant {
                        min[step.property] = step.constant;
                    } else {
                        continue 'queue;
                    }
                }
            }
            act(&mut queue, fallback, min, max)
        }
        accepted
    };

    println!("{}", part1);
    println!("{}", part2);
}
