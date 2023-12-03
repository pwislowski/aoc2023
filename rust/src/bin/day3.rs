use std::collections::{HashSet, VecDeque};
use std::{env, fs::read_to_string};

fn main() {
    let argss: Vec<String> = env::args().collect();
    let inn = read_to_string(argss[1].as_str()).unwrap();

    let mut summ1 = 0;
    let mut summ2 = 0;

    let lines: Vec<&str> = inn.lines().collect();
    let rlimit = lines.len() as i32;
    let climit = lines[0].len() as i32;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for idx in 0..rlimit {
        let mut is_adjacent = false;
        let mut stack = String::new();
        let mut dq: VecDeque<char> = VecDeque::new();

        let curr = lines[idx as usize];

        for (ic, c) in curr.char_indices() {
            let _ = visited.insert((idx, ic as i32));

            if c.is_numeric() {
                stack.push(c);

                for row in [-1, 0, 1] {
                    for col in [-1, 0, 1] {
                        if col == 0 && row == 0 {
                            continue;
                        }

                        let nrow = idx + row;
                        let ncol = ic as i32 + col;

                        if nrow < 0 || nrow >= rlimit || ncol < 0 || ncol >= climit {
                            continue;
                        }

                        let c_check = lines[nrow as usize].chars().nth(ncol as usize).unwrap();
                        if c_check.is_numeric() || c_check == '.' {
                            continue;
                        }

                        is_adjacent = true;

                        // part2
                        if !dq.is_empty() {
                            continue;
                        }

                        let mut adj_cords: Option<(i32, i32)> = None;
                        'outer: for rr in [-1, 0, 1] {
                            for cc in [-1, 0, 1] {
                                if rr == 0 && cc == 0 {
                                    continue;
                                }

                                let nxt_row = nrow + rr;
                                let nxt_col = ncol + cc;

                                if nxt_row < 0
                                    || nxt_row >= rlimit
                                    || nxt_col < 0
                                    || nxt_col >= climit
                                {
                                    continue;
                                }

                                if visited.contains(&(nxt_row, nxt_col)) {
                                    continue;
                                }

                                let nxt_char = lines[nxt_row as usize]
                                    .chars()
                                    .nth(nxt_col as usize)
                                    .unwrap();

                                if nxt_char.is_numeric() {
                                    adj_cords = Some((nxt_row, nxt_col));
                                    dq.push_back(nxt_char);
                                    break 'outer;
                                }
                            }
                        }

                        if let Some((zr, zc)) = adj_cords {
                            // go left
                            let mut nxt_col = zc - 1;
                            let mut skip_if_visted = false;
                            while nxt_col >= 0 && nxt_col < climit {
                                let nxt_char =
                                    lines[zr as usize].chars().nth(nxt_col as usize).unwrap();

                                if !nxt_char.is_numeric() {
                                    break;
                                }

                                if visited.contains(&(zr, nxt_col)) {
                                    dq.clear();
                                    skip_if_visted = true;
                                    break;
                                }

                                dq.push_front(nxt_char);
                                nxt_col -= 1;
                            }

                            // go right
                            let mut nxt_col = zc + 1;
                            while nxt_col >= 0 && nxt_col < climit && !skip_if_visted {
                                let nxt_char =
                                    lines[zr as usize].chars().nth(nxt_col as usize).unwrap();

                                if !nxt_char.is_numeric() {
                                    break;
                                }

                                dq.push_back(nxt_char);
                                nxt_col += 1;
                            }
                        }
                    }
                }
            }

            if !c.is_numeric() || ic as i32 == climit - 1 {
                if is_adjacent {
                    let val: i32 = stack.parse().unwrap();
                    summ1 += val;

                    if !dq.is_empty() {
                        let p: i32 = String::from_iter(dq.clone()).parse().unwrap();

                        summ2 += val * p
                    }
                }

                is_adjacent = false;
                stack.clear();
                dq.clear();
            }
        }
    }

    println!("Part 1: {}", summ1);
    println!("Part 2: {}", summ2);
}
