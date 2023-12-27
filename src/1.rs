use std::io;

fn first_and_last<I>(mut iter: I) -> Option<(I::Item, I::Item)>
where
    I: Iterator,
    I::Item: Copy,
{
    let first = iter.next()?;
    let last = iter.last().unwrap_or(first);
    Some((first, last))
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for res in io::stdin().lines() {
        let line = res.unwrap();
        {
            let (first, last) =
                first_and_last(&mut line.chars().filter_map(|d| d.to_digit(10))).unwrap();
            part1 += first * 10 + last;
        }
        {
            let (first, last) = first_and_last(&mut line.char_indices().filter_map(|(i, d)| {
                d.to_digit(10).or_else(|| {
                    [
                        "zero", "one", "two", "three", "four",
                        "five", "six", "seven", "eight", "nine",
                    ]
                    .iter()
                    .enumerate()
                    .find_map(|(n, prefix)| line[i..].starts_with(prefix).then_some(n as u32))
                })
            }))
            .unwrap();
            part2 += first * 10 + last;
        }
    }
    println!("{}", part1);
    println!("{}", part2);
}
