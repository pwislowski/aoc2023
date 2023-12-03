use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let argss: Vec<String> = env::args().collect();
    let inn = read_to_string(argss[1].as_str()).unwrap();
    let map: HashMap<&str, i32> = HashMap::from_iter([("red", 12), ("green", 13), ("blue", 14)]);

    let mut summ: i32 = 0;
    let mut summ2: i32 = 0;
    for line in inn.lines() {
        let (rg, rb) = line.split_once(':').unwrap();
        let game = rg.replace("Game ", "").parse::<i32>().unwrap();

        let mut skip_p1 = false;
        let mut fewest: HashMap<&str, i32> = HashMap::new();
        for set in rb.split(';') {
            for revealed in set.split(',') {
                let (rq, c) = revealed.trim().split_once(' ').unwrap();
                let q: i32 = rq.parse().unwrap();

                if q > *map.get(c).unwrap() {
                    skip_p1 = true;
                }

                if let Some(v) = fewest.get(c) {
                    if q > *v {
                        let _ = fewest.insert(c, q);
                    }
                } else {
                    let _ = fewest.insert(c, q);
                }
            }
        }

        if !skip_p1 {
            summ += game;
        }

        let prod: i32 = fewest.values().product();
        summ2 += prod;
    }

    println!("Part 1: {}", summ);
    println!("Part 2: {}", summ2);
}
