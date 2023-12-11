use aoc2023::read_file;
use std::collections::HashMap;

fn main() {
    let inn = read_file();
    let mut lines = inn.lines();
    let instr: Vec<char> = lines.next().unwrap().chars().collect();
    let _ = lines.next().unwrap();

    let nodes: HashMap<String, (String, String)> = lines
        .map(|line| {
            let (k, tup) = line.split_once(" = ").unwrap();

            let cleaned = tup.replace("(", "").replace(")", "");
            let choices: Vec<String> = cleaned.split(", ").map(|x| x.to_string()).collect();
            let a = choices[0].to_owned();
            let b = choices[1].to_owned();

            (k.to_owned(), (a, b))
        })
        .collect();

    solve_p1(&instr, &nodes);
    solve_p2(&instr, &nodes);
}

fn solve_p1(instr: &[char], nodes: &HashMap<String, (String, String)>) {
    let n = instr.len();

    let mut curr_node = "AAA";
    let mut curr_steps = 0;

    loop {
        if curr_node == "ZZZ" {
            println!("Part 1: {}", curr_steps);
            break;
        }

        let side = instr[curr_steps % n];
        curr_node = {
            if side == 'L' {
                nodes.get(curr_node).unwrap().0.as_str()
            } else {
                nodes.get(curr_node).unwrap().1.as_str()
            }
        };

        curr_steps += 1;
    }
}

fn solve_p2(instr: &[char], nodes: &HashMap<String, (String, String)>) {
    let start_nodes: Vec<String> = nodes
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_owned())
        .collect();

    let ghosts: Vec<u64> = start_nodes
        .iter()
        .map(|node| {
            let mut curr_node = node.as_str();
            let mut curr_steps = 0;

            loop {
                if curr_node.ends_with("Z") {
                    break;
                }

                let side = instr[curr_steps % instr.len()];
                curr_node = {
                    if side == 'L' {
                        nodes.get(curr_node).unwrap().0.as_str()
                    } else {
                        nodes.get(curr_node).unwrap().1.as_str()
                    }
                };

                curr_steps += 1;
            }

            curr_steps as u64
        })
        .collect();

    let ans = find_range_lcm(&ghosts);

    println!("Part 2: {}", ans);
}

fn greatest_common_demoniator(x: u64, y: u64) -> u64 {
    let mut num1 = x;
    let mut num2 = y;

    while num2 != 0 {
        let temp = num1;
        num1 = num2;
        num2 = temp % num2;
    }

    num1
}

fn least_common_multiplier(x: u64, y: u64) -> u64 {
    x * y / greatest_common_demoniator(x, y)
}

fn find_range_lcm(lst: &[u64]) -> u64 {
    let mut lcm = lst[0];

    for num in lst.iter().skip(1) {
        lcm = least_common_multiplier(lcm, *num);
    }

    lcm
}
