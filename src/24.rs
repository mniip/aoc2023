use std::io::stdin;

use regex::Regex;

fn main() {
    let line_re =
        Regex::new(r"^([\d-]+), *([\d-]+), *([\d-]+) *@ *([\d-]+), *([\d-]+), *([\d-]+)$").unwrap();
    let input: Vec<((i128, i128, i128), (i128, i128, i128))> = stdin()
        .lines()
        .map(|res| {
            let line = res.unwrap();
            let [x, y, z, vx, vy, vz] = line_re
                .captures(&line)
                .unwrap()
                .extract()
                .1
                .map(|s| str::parse(s).unwrap());
            ((x, y, z), (vx, vy, vz))
        })
        .collect();

    const MIN: i128 = 200000000000000;
    const MAX: i128 = 400000000000000;
    let part1 = input
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, (p1, v1))| {
            input[i + 1..]
                .iter()
                .copied()
                .map(move |(p2, v2)| (p1, v1, p2, v2))
        })
        .filter(
            |&((x1, y1, _), (vx1, vy1, _), (x2, y2, _), (vx2, vy2, _))| {
                let det = vy1 * vx2 - vx1 * vy2;
                if det == 0 {
                    return false;
                }
                let t1 = ((y2 - y1) * vx2 - (x2 - x1) * vy2) * det.signum();
                let t2 = ((y2 - y1) * vx1 - (x2 - x1) * vy1) * det.signum();
                let det = det.abs();
                t1 >= 0
                    && t2 >= 0
                    && t1 * vx1 >= (MIN - x1) * det
                    && t1 * vx1 <= (MAX - x1) * det
                    && t1 * vy1 >= (MIN - y1) * det
                    && t1 * vy1 <= (MAX - y1) * det
            },
        )
        .count();

    let part2 = {
        let ((x, y, z), _) = (0i128..)
            .flat_map(|total| {
                (-total..=total).flat_map(move |vx| {
                    (vx.abs() - total..=total - vx.abs()).flat_map(move |vy| {
                        [vx.abs() + vy.abs() - total, total - vx.abs() - vy.abs()]
                            .map(move |vz| (vx, vy, vz))
                    })
                })
            })
            .filter_map(|(vx, vy, vz)| {
                let ((x1, y1, z1), (vx1, vy1, vz1), (x2, y2, z2), (vx2, vy2, vz2), det) = input
                    .iter()
                    .copied()
                    .enumerate()
                    .flat_map(|(i, (p1, v1))| {
                        input[i + 1..]
                            .iter()
                            .copied()
                            .map(move |(p2, v2)| (p1, v1, p2, v2))
                    })
                    .find_map(|(p1, v1 @ (vx1, vy1, _), p2, v2 @ (vx2, vy2, _))| {
                        let det = (vx1 - vx) * (vy - vy2) - (vy1 - vy) * (vx - vx2);
                        (det != 0).then_some((p1, v1, p2, v2, det))
                    })
                    .unwrap();
                let t1 = ((vy - vy2) * (x2 - x1) + (vx2 - vx) * (y2 - y1)) * det.signum();
                let t2 = ((vy - vy1) * (x2 - x1) + (vx1 - vx) * (y2 - y1)) * det.signum();
                let det = det.abs();
                if t1 * (vz1 - vz) + t2 * (vz - vz2) != (z2 - z1) * det {
                    return None;
                }
                if t1 * (vx1 - vx) % det != 0
                    || t1 * (vy1 - vy) % det != 0
                    || t1 * (vz1 - vz) % det != 0
                {
                    return None;
                }
                let x = x1 + t1 * (vx1 - vx) / det;
                let y = y1 + t1 * (vy1 - vy) / det;
                let z = z1 + t1 * (vz1 - vz) / det;
                if !input.iter().all(|&((xn, yn, zn), (vxn, vyn, vzn))| {
                    (xn - x) * (vyn - vy) == (yn - y) * (vxn - vx)
                        && (xn - x) * (vzn - vz) == (zn - z) * (vxn - vx)
                }) {
                    return None;
                };
                Some(((x, y, z), (vx, vy, vz)))
            })
            .next()
            .unwrap();
        x + y + z
    };

    println!("{}", part1);
    println!("{}", part2);
}
