use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let map: HashMap<&str, i32> = HashMap::from_iter([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let argss: Vec<String> = env::args().collect();
    let fname = argss[1].as_str();
    let inn = read_to_string(fname).unwrap();

    let mut summ: i32 = 0;

    for line in inn.lines() {
        let front = line.chars().find(|x| x.is_numeric()).unwrap();
        let back = line.chars().rfind(|x| x.is_numeric()).unwrap();

        let digit = format!("{}{}", front, back).parse::<i32>().unwrap();
        summ += digit;
    }

    println!("Part 1: {}", summ);

    summ = 0;

    for line in inn.lines() {
        let mut temp_vec: Vec<i32> = Vec::new();
        let mut stack: String = String::new();

        for c in line.chars() {
            stack.push(c);

            let mut found = false;
            for key in map.keys() {
                if stack.contains(*key) {
                    let v = *map.get(*key).unwrap();
                    temp_vec.push(v);

                    stack.clear();
                    stack.push(c);
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }

            if c.is_numeric() {
                let v = c.to_string().parse::<i32>().unwrap();
                temp_vec.push(v);
                stack.clear();
                continue;
            }
        }
        let first = *temp_vec.first().unwrap();
        let last = *temp_vec.last().unwrap();

        let digit = first * 10 + last;

        summ += digit;
    }

    println!("Part 2: {}", summ);
}
