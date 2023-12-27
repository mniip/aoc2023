use std::io::stdin;

use itertools::Itertools;

fn char_to_rank(ch: char) -> Option<u8> {
    match ch {
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}

const JOKER: u8 = 1;

fn jack_for_joker(r: u8) -> u8 {
    match r {
        11 => JOKER,
        _ => r,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn rank_hand(hand: [u8; 5]) -> HandRank {
    let mut non_jokers = hand.into_iter().filter(|&r| r != JOKER).collect::<Vec<_>>();
    non_jokers.sort();
    let mut runs = non_jokers
        .into_iter()
        .group_by(|&r| r)
        .into_iter()
        .map(|(_, run)| run.count())
        .collect::<Vec<usize>>();
    runs.sort();
    match runs[..] {
        [] | [_] => HandRank::FiveOfAKind,
        [1, _] => HandRank::FourOfAKind,
        [2, _] => HandRank::FullHouse,
        [1, 1, _] => HandRank::ThreeOfAKind,
        [1, 2, 2] => HandRank::TwoPair,
        [1, 1, 1, _] => HandRank::OnePair,
        [1, 1, 1, 1, 1] => HandRank::HighCard,
        _ => panic!(),
    }
}

fn main() {
    let input: Vec<([u8; 5], u32)> = stdin()
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let (hand, bid) = line.split_once(' ').unwrap();
            (
                hand.chars()
                    .filter_map(char_to_rank)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                bid.parse().unwrap(),
            )
        })
        .collect();

    let score = |mut input: Vec<([u8; 5], u32)>| {
        input.sort_by_key(|&(hand, _)| (rank_hand(hand), hand));
        input
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) as u32 * bid)
            .sum::<u32>()
    };

    let part1 = score(input.clone());
    let part2 = score(
        input
            .into_iter()
            .map(|(hand, bid)| (hand.map(jack_for_joker), bid))
            .collect(),
    );

    println!("{}", part1);
    println!("{}", part2);
}
