use aoc2023::read_file;
use std::collections::HashMap;

fn main() {
    let inn = read_file();
    let mut p1 = 0;
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for line in inn.lines() {
        let (card, nums) = line.split_once(':').unwrap();

        let card_no: i32 = card.split_whitespace().last().unwrap().parse().unwrap();
        *counter.entry(card_no).or_insert(0) += 1;
        let copies = counter.get(&card_no).unwrap().to_owned();

        let (l, r) = nums.split_once('|').unwrap();

        let mut count = 0;
        let mut left: Vec<&str> = Vec::new();

        for val in l.trim().split_whitespace() {
            left.push(val.trim());
        }

        for val in r.trim().split_whitespace() {
            if left.contains(&val.trim()) {
                count += 1;
            }
        }

        let mut new_card: i32;
        let mut points = 0;
        for c in 0..count {
            new_card = card_no + c + 1;
            *counter.entry(new_card).or_insert(0) += copies;

            if c == 0 {
                points += 1;
                continue;
            }

            points *= 2;
        }

        p1 += points;
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", counter.values().sum::<i32>());
}
