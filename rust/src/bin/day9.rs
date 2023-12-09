use aoc2023::read_file;

fn main() {
    let inn = read_file();

    let mut p1 = 0;
    let mut p2 = 0;

    for line in inn.lines() {
        let arr: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let mut diffs: Vec<i32> = Vec::new();
        let mut fdiffs: Vec<i32> = Vec::new();
        let mut curr_arr = arr.clone();
        loop {
            let mut t: Vec<i32> = Vec::new();

            for chunk in curr_arr.windows(2) {
                let num1 = chunk[0];
                let num2 = chunk[1];
                t.push(num2 - num1);
            }

            let all_nuls = t.iter().all(|x| *x == 0);

            let last_diff = *t.last().unwrap();
            diffs.push(last_diff);

            let first_diff = *t.first().unwrap();
            fdiffs.push(first_diff);

            curr_arr = t;

            if all_nuls {
                break;
            }
        }

        let mut ep = 0;
        for p in diffs.iter().rev() {
            ep += *p;
        }

        p1 += ep + arr.last().unwrap();

        let mut fep = 0;
        for d in fdiffs.iter().rev() {
            fep = d - fep;
        }

        p2 += arr.first().unwrap() - fep;
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
