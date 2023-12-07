// ! FAILING

use aoc2023::read_file;
use std::collections::HashMap;

fn main() {
    let inn = read_file();

    // key_mapping
    let mut key_p1: HashMap<&str, i32> =
        HashMap::from_iter([("A", 14), ("K", 13), ("Q", 12), ("J", 11), ("T", 10)]);

    let mut key_p2: HashMap<&str, i32> =
        HashMap::from_iter([("A", 14), ("K", 13), ("Q", 12), ("J", 1), ("T", 10)]);

    key_p1.insert("2", 2);
    key_p1.insert("3", 3);
    key_p1.insert("4", 4);
    key_p1.insert("5", 5);
    key_p1.insert("6", 6);
    key_p1.insert("7", 7);
    key_p1.insert("8", 8);
    key_p1.insert("9", 9);

    key_p2.insert("2", 2);
    key_p2.insert("3", 3);
    key_p2.insert("4", 4);
    key_p2.insert("5", 5);
    key_p2.insert("6", 6);
    key_p2.insert("7", 7);
    key_p2.insert("8", 8);
    key_p2.insert("9", 9);

    // list tables
    let mut tab_p1: HashMap<&str, Vec<String>> = HashMap::new();
    let mut tab_p2: HashMap<&str, Vec<String>> = HashMap::new();

    let combinations = [
        "fiveok", "fourok", "fullh", "threeok", "twopair", "onepair", "hc",
    ];
    for key in combinations {
        tab_p1.insert(key, vec![]);
        tab_p2.insert(key, vec![]);
    }

    // bid table
    let mut bid_map: HashMap<&str, i32> = HashMap::new();

    for line in inn.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        bid_map.insert(hand, bid.parse().unwrap());

        #[allow(non_snake_case)]
        let mut C: HashMap<char, usize> = HashMap::new();
        for c in hand.chars() {
            *C.entry(c).or_insert(0) += 1;
        }

        let p1_range: Vec<usize> = C.values().map(|x| x.clone()).collect();
        let mut p2_range: Vec<usize> = Vec::new();

        let mut maxx: usize = 0;
        let mut mididx: usize = 0;
        let mut jkr_cnt: usize = 0;
        for (idx, (key, v)) in C.iter().enumerate() {
            if *key == 'J' {
                jkr_cnt += 1;
                continue;
            }

            if maxx < *v {
                maxx = *v;
                mididx = idx;
            }

            p2_range.push(*v);
        }

        // Par 2 Relevant:  if all jokers
        if let Some(v) = p2_range.get_mut(mididx) {
            *v += jkr_cnt;
        } else {
            tab_p2.get_mut("fiveok").unwrap().push(hand.to_string());
            continue;
        }

        if p1_range.contains(&5) {
            tab_p1.get_mut("fiveok").unwrap().push(hand.to_string());
        } else if p1_range.contains(&4) {
            tab_p1.get_mut("fourok").unwrap().push(hand.to_string());
        } else if p1_range.contains(&3) && p1_range.contains(&2) {
            tab_p1.get_mut("fullh").unwrap().push(hand.to_string());
        } else if p1_range.contains(&3) {
            tab_p1.get_mut("threeok").unwrap().push(hand.to_string());
        } else if &p1_range.iter().filter(|x| x == &&2).count() == &2 {
            tab_p1.get_mut("twopair").unwrap().push(hand.to_string());
        } else if p1_range.contains(&2) {
            tab_p1.get_mut("onepair").unwrap().push(hand.to_string());
        } else {
            tab_p1.get_mut("hc").unwrap().push(hand.to_string());
        }

        if p2_range.contains(&5) {
            tab_p2.get_mut("fiveok").unwrap().push(hand.to_string());
        } else if p2_range.contains(&4) {
            tab_p2.get_mut("fourok").unwrap().push(hand.to_string());
        } else if p2_range.contains(&3) && p2_range.contains(&2) {
            tab_p2.get_mut("fullh").unwrap().push(hand.to_string());
        } else if p2_range.contains(&3) {
            tab_p2.get_mut("threeok").unwrap().push(hand.to_string());
        } else if &p2_range.iter().filter(|x| x == &&2).count() == &2 {
            tab_p2.get_mut("twopair").unwrap().push(hand.to_string());
        } else if p2_range.contains(&2) {
            tab_p2.get_mut("onepair").unwrap().push(hand.to_string());
        } else {
            tab_p2.get_mut("hc").unwrap().push(hand.to_string());
        }
    }

    let mut p1: i32 = 0;
    let mut p2: i32 = 0;
    for comb in combinations.iter().rev() {
        let p1_srted = merge_sort(tab_p1.get(*comb).unwrap().to_owned(), &key_p1);
        let p2_srted = merge_sort(tab_p2.get(*comb).unwrap().to_owned(), &key_p2);
        println!("{:?}", p1_srted);

        for (rank, hand) in p1_srted.iter().enumerate() {
            p1 += (rank + 1) as i32 * bid_map.get(hand.as_str()).unwrap();
        }

        for (rank, hand) in p2_srted.iter().enumerate() {
            p2 += (rank + 1) as i32 * bid_map.get(hand.as_str()).unwrap();
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn merge_sort(lst: Vec<String>, key: &HashMap<&str, i32>) -> Vec<String> {
    let len = lst.len();

    if len < 2 {
        return lst;
    }

    let mid = len / 2;

    let mut lpart: Vec<String> = Vec::new();
    let mut rpart: Vec<String> = Vec::new();

    for (idx, v) in lst.iter().enumerate() {
        if idx < mid {
            lpart.push(v.clone());
        } else {
            rpart.push(v.clone());
        }
    }

    println!("{:?}", lpart);
    println!("{:?}", rpart);

    let left = merge_sort(lpart, key);
    let right = merge_sort(rpart, key);

    let mut temp: Vec<String> = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        let lhand = left[i].as_str();
        let rhand = right[i].as_str();

        let lpiped: Vec<i32> = lhand
            .chars()
            .map(|x| *key.get(x.to_string().as_str()).unwrap())
            .collect();
        let rpiped: Vec<i32> = rhand
            .chars()
            .map(|x| *key.get(x.to_string().as_str()).unwrap())
            .collect();

        for (lv, rv) in lpiped.iter().zip(rpiped.iter()) {
            if lv < rv {
                temp.push(lhand.to_string());
                i += 1;
                break;
            }

            if rv < lv {
                temp.push(rhand.to_string());
                j += 1;
                break;
            }
        }
    }

    while i < left.len() {
        temp.push(left[i].clone());
        i += 1;
    }

    while j < right.len() {
        temp.push(right[j].clone());
        j += 1;
    }

    return temp;
}
