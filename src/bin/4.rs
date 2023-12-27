use regex::Regex;
use std::io::stdin;

fn main() {
    let card_re = Regex::new(r"^Card\s+\d+:(.*)\|(.*)$").unwrap();
    let matches: Vec<usize> = stdin()
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let [winning_str, have_str] = card_re.captures(&line).unwrap().extract().1;
            let winning = winning_str
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i32>, _>>()
                .unwrap();
            have_str
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .filter(|&n| winning.contains(&n))
                .count()
        })
        .collect();
    let part1 = matches.iter().map(|m| 1 << m >> 1).sum::<usize>();
    let mut copies = vec![1; matches.len()];
    for (i, m) in matches.into_iter().enumerate() {
        let copy = copies[i];
        for j in 1..=m {
            copies[i + j] += copy
        }
    }
    let part2 = copies.into_iter().sum::<usize>();
    println!("{}", part1);
    println!("{}", part2);
}
