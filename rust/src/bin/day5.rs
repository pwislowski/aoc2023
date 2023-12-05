use aoc2023::read_file;
// use std::cmp;

fn main() {
    let inn = read_file();
    let mut lines = inn.split("\n\n");
    let (_, _seeds) = lines.next().unwrap().split_once(':').unwrap();
    let mut p1: Vec<i64> = _seeds
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut p2: Vec<i64> = Vec::new();
    for i in 0..p1.len() {
        if i % 2 != 0 {
            continue;
        }
        let start = p1[i];
        let range = p1[i + 1];

        for z in 0..range {
            p2.push(start + z);
        }
    }

    // todo: optimize wip
    // let mut p2: Vec<(i64, i64)> = Vec::new();
    // for i in 0..p1.len() {
    //     if i % 2 != 0 {
    //         continue;
    //     }
    //     let start = p1[i];
    //     let range = p1[i + 1];
    //     p2.push((start, range));
    // }

    println!("{:?}", p2);

    for chunk in lines {
        let mut new_p1 = p1.clone();
        let mut new_p2 = p2.clone();

        for line in chunk.lines() {
            let fchar = line.chars().nth(0).unwrap();

            if !fchar.is_numeric() {
                continue;
            }

            let inst: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            println!("\n curr line: {:?}", inst);

            let src = inst[1];
            let dest = inst[0];
            let steps = inst[2];

            for i in 0..p1.len() {
                let cval = p1[i];

                if src <= cval && cval < src + steps {
                    new_p1[i] = dest + (cval - src);
                }
            }

            for i in 0..p2.len() {
                let cval = p2[i];

                if src <= cval && cval < src + steps {
                    new_p2[i] = dest + (cval - src);
                }
            }

            // todo: optimize wip
            // let src_rb = src + steps - 1;
            // let lag = dest - src;
            // for i in 0..p2.len() {
            //     let (ref_lb, ssteps) = p2[i];
            //     let ref_rb = ref_lb + ssteps - 1;

            //     if src_rb < ref_lb || ref_rb < src {
            //         continue;
            //     }

            //     let new_lb = cmp::max(src, ref_lb);
            //     let new_rb = cmp::min(src_rb, ref_rb);

            //     println!("intersection");
            //     println!("{} -> {}", new_lb, new_rb);
            //     println!("lag: {lag}");

            //     let new_steps = new_rb - new_lb + 1;

            //     new_p2[i] = (new_lb + lag, new_steps);
            // }
        }

        p1 = new_p1;
        p2 = new_p2;
        println!("{:?}", p2);
    }

    println!("Part 1: {}", p1.iter().min().unwrap());
    println!("Part 2: {}", p2.iter().min().unwrap());

    // todo: optimize wip
    // let mut minn = i64::MAX;
    // for (s, _) in p2 {
    //     if s < minn {
    //         minn = s
    //     }
    // }
    // println!("Part 2: {}", minn);
}
