use std::io::stdin;

fn main() {
    let numbers = stdin()
        .lines()
        .map(|res| {
            res.unwrap()
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i32>, _>>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    fn extrapolate(mut row: Vec<i32>) -> i32 {
        let mut i = 0;
        loop {
            let mut prev = 0;
            for x in row[i..].iter_mut() {
                (*x, prev) = (*x - prev, *x);
            }
            if row[i..].iter().copied().all(|x| x == 0) { break }
            i += 1;
        }
        row.push(0);
        loop {
            let mut total = 0;
            for x in row[i..].iter_mut() {
                *x += total;
                total = *x;
            }
            if i == 0 { break }
            i -= 1;
        }
        row.pop().unwrap()
    }

    let part1 = numbers.iter().cloned().map(extrapolate).sum::<i32>();
    let part2 = numbers.into_iter().map(|mut row| {
        row.reverse();
        extrapolate(row)
    }).sum::<i32>();

    println!("{}", part1);
    println!("{}", part2);
}
