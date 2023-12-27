use std::io::stdin;

fn main() {
    let get_number_line = || {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let numbers = line
            .split_ascii_whitespace()
            .skip(1)
            .map(str::parse)
            .collect::<Result<Vec<i64>, _>>()
            .unwrap();
        line.retain(|c| c.is_ascii_digit());
        (numbers, line.parse::<i64>().unwrap())
    };
    let (times, time) = get_number_line();
    let (distances, distance) = get_number_line();

    let solve = |(t, d)| {
        let q = ((t * t - 4 * d) as f64).sqrt() / 2.0;
        let m = (t as f64) / 2.0;
        let min = (m - q).floor() as i64 + 1;
        let max = (m + q).ceil() as i64 - 1;
        max - min + 1
    };

    let part1 = times
        .iter()
        .copied()
        .zip(distances)
        .map(solve)
        .product::<i64>();

    let part2 = solve((time, distance));

    println!("{}", part1);
    println!("{}", part2);
}
