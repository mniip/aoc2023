use std::io::stdin;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tri {
    Good,
    Damaged,
    Unknown,
}

fn main() {
    let input: Vec<(Vec<Tri>, Vec<usize>)> = stdin()
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let (row, clues) = line.split_once(" ").unwrap();
            (
                row.chars()
                    .map(|c| match c {
                        '.' => Tri::Good,
                        '#' => Tri::Damaged,
                        '?' => Tri::Unknown,
                        _ => panic!(),
                    })
                    .collect(),
                clues
                    .split(",")
                    .map(str::parse)
                    .collect::<Result<_, _>>()
                    .unwrap(),
            )
        })
        .collect();

    fn solutions(row: &[Tri], clues: &[usize]) -> u64 {
        let mut counts: Vec<Vec<u64>> = vec![vec![0; row.len() + 1]; clues.len() + 1];
        counts[0][0] = 1;
        for j in 1..=row.len() {
            counts[0][j] = if row[j - 1] != Tri::Damaged {
                counts[0][j - 1]
            } else {
                0
            }
        }
        for i in 1..=clues.len() {
            let clue = clues[i - 1];
            counts[i][0] = 0;
            for j in 1..=row.len() {
                counts[i][j] = if row[j - 1] != Tri::Damaged {
                    counts[i][j - 1]
                } else {
                    0
                } + if j > clue
                    && row[j - clue - 1] != Tri::Damaged
                    && row[j - clue..j].iter().copied().all(|t| t != Tri::Good)
                {
                    counts[i - 1][j - clue - 1]
                } else if j == clue && row[..clue].iter().copied().all(|t| t != Tri::Good) {
                    counts[i - 1][0]
                } else {
                    0
                }
            }
        }
        counts[clues.len()][row.len()]
    }

    let part1 = input
        .iter()
        .map(|(row, clues)| solutions(row, clues))
        .sum::<u64>();

    let part2 = input
        .iter()
        .map(|(row, clues)| {
            solutions(
                &[&[Tri::Unknown][..], &row].concat().repeat(5)[1..],
                &clues.repeat(5),
            )
        })
        .sum::<u64>();

    println!("{}", part1);
    println!("{}", part2);
}
