use std::{io::stdin, str::FromStr};

fn main() {
    let input: Vec<String> = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect();

    fn hash(value: &str) -> u8 {
        value
            .chars()
            .fold(0, |x, c| x.wrapping_add(c as u8).wrapping_mul(17))
    }

    let part1 = input.iter().map(|s| hash(s) as u32).sum::<u32>();

    enum Step {
        Insert(String, u32),
        Delete(String),
    }

    impl FromStr for Step {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some((key, value)) = s.split_once('=') {
                Ok(Step::Insert(
                    String::from(key),
                    value.parse().map_err(|_| ())?,
                ))
            } else if let Some((key, "")) = s.split_once('-') {
                Ok(Step::Delete(String::from(key)))
            } else {
                Err(())
            }
        }
    }

    let steps = input
        .iter()
        .map(|s| str::parse(s))
        .collect::<Result<Vec<Step>, _>>()
        .unwrap();

    let mut hashmap: [Vec<(String, u32)>; 256] = [(); 256].map(|_| Vec::new());
    for step in steps {
        match step {
            Step::Delete(key) => hashmap[hash(&key) as usize].retain(|(k, _)| k != &key),
            Step::Insert(key, value) => {
                let cell = &mut hashmap[hash(&key) as usize];
                match cell.iter_mut().find(|(k, _)| k == &key) {
                    Some((_, v)) => *v = value,
                    None => cell.push((key, value)),
                }
            }
        }
    }
    let part2 = hashmap
        .into_iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(move |(j, (_, f))| (i + 1) as u32 * (j + 1) as u32 * f)
        })
        .sum::<u32>();

    println!("{}", part1);
    println!("{}", part2);
}
