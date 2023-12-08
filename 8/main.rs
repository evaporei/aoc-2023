use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // let mut lines = read_lines("./example1").unwrap(); // 2
    // let mut lines = read_lines("./example2").unwrap(); // 6
    // let mut lines = read_lines("./example_part_two").unwrap(); // 6
    let mut lines = read_lines("./input").unwrap(); // 18157, 19783 (too low),
    // 201405124411 (lcm, wrong ;-;), 14299763833181 (yay!)

    // LLR
    let steps = lines.next().unwrap().unwrap();

    // empty line
    let _ = lines.next();

    let mut map = BTreeMap::new();

    for line in lines {
        // CCC = (BBB, DDD)
        let line = line.unwrap();
        let mut line = line.split('=');

        let mut origin = line.next().unwrap().to_owned();
        origin.pop(); // \s

        let mut directions = line.next().unwrap().split(',');
        let mut left = directions.next().unwrap().to_owned();
        let mut right = directions.next().unwrap().to_owned();

        left.remove(0); // (
        left.remove(0); // \s
        right.remove(4); // )
        right.remove(0); // \s

        // just insert the first one
        map.entry(origin)
            .or_insert((left, right));
    }

    let a_steps: Vec<String> = map
        .keys()
        .filter(|s| s.ends_with('A')).
        map(|s| s.clone())
        .collect();
    let z_steps: Vec<String> = map
        .keys()
        .filter(|s| s.ends_with('Z'))
        .map(|s| s.clone())
        .collect();

    for z in &z_steps {
        map.remove(z);
    }

    let n_steps = find_steps("AAA", &["ZZZ".to_string()], &map, &steps);
    println!("part one {n_steps}");

    let mut everyone_at_z = 1;
    for a_step in a_steps {
        let n_steps = find_steps(&a_step, &z_steps, &map, &steps);
        everyone_at_z = lcm(everyone_at_z, n_steps);
    }
    println!("part two: {everyone_at_z}");
}

fn find_steps(start: &str, ends: &[String], map: &BTreeMap<String, (String, String)>, steps: &str) -> u64 {
    let (mut l, mut r) = map.get(start).unwrap().clone();
    let mut n_steps = 1;
    let mut i = 0;

    while !ends.contains(&l) || !ends.contains(&r) {
        if i == steps.len() {
            i = 0;
        }
        let step = steps.bytes().nth(i).unwrap();
        (l, r) = match step {
            b'L' => match map.get(&l) {
                Some(directions) => directions.clone(),
                None => break,
            },
            b'R' => match map.get(&r) {
                Some(directions) => directions.clone(),
                None => break,
            },
            _ => unreachable!("bad input, only L and R are allowed"),
        };
        n_steps += 1;
        i += 1;
    }

    n_steps
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
