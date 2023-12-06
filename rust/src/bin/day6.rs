use aoc2023::read_file;
use std::collections::HashMap;

fn main() {
    let inn = read_file();
    let mut lines = inn.lines();

    let time: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let dist: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut p1 = 1;
    for (t, d) in time.iter().zip(dist.iter()) {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for speed in 0..=*t {
            let travelled = speed * (t - speed);
            map.insert(speed, travelled);
        }

        let ways = map.values().filter(|x| x > &d).count();
        p1 *= ways as i32;
    }

    println!("Part 1: {}", p1);

    let t_joined: i64 = String::from_iter(time.iter().map(|x| x.to_string()))
        .parse()
        .unwrap();
    let d_joined: i64 = String::from_iter(dist.iter().map(|x| x.to_string()))
        .parse()
        .unwrap();

    // todo: optimization idea: distribution cut-off
    let mut map: HashMap<i64, i64> = HashMap::new();
    for speed in 0..=t_joined {
        let travelled = speed * (t_joined - speed);
        map.insert(speed, travelled);
    }

    println!(
        "Part 2: {}",
        map.values().filter(|x| *x > &d_joined).count()
    );
}
