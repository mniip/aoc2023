use regex::Regex;
use std::{error::Error, io, str::FromStr};

#[derive(Clone, Copy, Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

const ZERO_ROUND: Round = Round {
    red: 0,
    green: 0,
    blue: 0,
};

fn max_rounds(r1: Round, r2: Round) -> Round {
    return Round {
        red: Ord::max(r1.red, r2.red),
        green: Ord::max(r1.green, r2.green),
        blue: Ord::max(r1.blue, r2.blue),
    };
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(g: &str) -> Result<Self, Self::Err> {
        let game_re = Regex::new(r"^Game (\d+):(.*)$")?;
        let ball_re = Regex::new(r"^\s*(\d+)\s+(\w+)\s*$")?;
        let [id_str, rest] = game_re
            .captures(&g)
            .ok_or("Could not parse game")?
            .extract()
            .1;
        let id = id_str.parse()?;
        let rounds = rest
            .split(';')
            .map(|r| {
                let mut round = ZERO_ROUND;
                for ball in r.split(',') {
                    let [num_str, color] = ball_re
                        .captures(ball)
                        .ok_or("Could not parse ball")?
                        .extract()
                        .1;
                    let num = num_str.parse::<i32>()?;
                    match color {
                        "red" => round.red += num,
                        "green" => round.green += num,
                        "blue" => round.blue += num,
                        _ => return Err(("Unknown color: ".to_string() + color).into()),
                    }
                }
                Ok(round)
            })
            .collect::<Result<_, Self::Err>>()?;
        Ok(Game { id, rounds })
    }
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for res in io::stdin().lines() {
        let game = res.unwrap().parse::<Game>().unwrap();
        if game
            .rounds
            .iter()
            .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
        {
            part1 += game.id;
        }
        let Round { red, green, blue } = game
            .rounds
            .iter()
            .fold(ZERO_ROUND, |r1, &r2| max_rounds(r1, r2));
        part2 += red * green * blue
    }
    println!("{}", part1);
    println!("{}", part2);
}
