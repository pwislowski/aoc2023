use aoc2023::read_file;
use std::collections::HashSet;

fn main() {
    let inn = read_file();
    let grid: Vec<Vec<char>> = inn.lines().map(|line| line.chars().collect()).collect();
    let p1 = solve(&grid, 2);
    let p2 = solve(&grid, 1_000_000);

    println!("Print 1: {p1}");
    println!("Print 2: {p2}");
}

fn solve(rows: &[Vec<char>], spacing: usize) -> usize {
    let rng = rows.len();
    let mut cols: Vec<Vec<char>> = Vec::new();
    for i in 0..rng {
        let mut temp: Vec<char> = Vec::new();
        for row in rows {
            let to_add = row[i];
            temp.push(to_add);
        }

        cols.push(temp);
    }

    let mut ys: Vec<usize> = Vec::new();
    let mut temp = 0;
    for row in rows {
        if row.contains(&'#') {
            temp += 1;
        } else {
            temp += spacing;
        }

        ys.push(temp);
    }

    let mut xs: Vec<usize> = Vec::new();
    temp = 0;
    for col in cols {
        if col.contains(&'#') {
            temp += 1;
        } else {
            temp += spacing;
        }

        xs.push(temp);
    }

    let mut points: Vec<(usize, usize)> = Vec::new();
    for y in 0..rng {
        for x in 0..rng {
            if rows[y][x] == '#' {
                points.push((xs[x], ys[y]));
            }
        }
    }

    let mut combinations: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for point in points.iter() {
        for other in points.iter() {
            if point == other {
                continue;
            }

            if combinations.contains(&(*point, *other)) || combinations.contains(&(*other, *point))
            {
                continue;
            }

            combinations.insert((*point, *other));
        }
    }

    combinations
        .iter()
        .map(|(l, r)| {
            let x0 = l.0;
            let y0 = l.1;
            let x1 = r.0;
            let y1 = r.1;

            x0.abs_diff(x1) + y0.abs_diff(y1)
        })
        .sum()
}
