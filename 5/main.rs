use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// input could be seed, soil, etc
fn dst_from_src(input: u32, dst: u32, src: u32, range: u32) -> u32 {
    if src > input || src + range < input {
        return input;
    }

    // it could be (input - src) == 0
    if src == input {
        return dst;
    }
    // for (dst_n, src_n) in (dst..dst+range).iter().zip((src..src+range).iter()) {
    // for dst_n in dst..dst+range {
    // }
    let diff = input - src;
    dst + diff
}

#[test]
fn test_src_to_dst() {
    let seed = 100;
    // 120 > 100, there's nothing to range, input returns
    assert_eq!(dst_from_src(seed, 13, 120, 50), 100);
    // 100 == 100, return first dst
    assert_eq!(dst_from_src(seed, 50, 100, 2), 50);
    // 97 + 2 = 99, still lower than 100, return input
    assert_eq!(dst_from_src(seed, 50, 97, 2), 100);

    // succeeds
    assert_eq!(dst_from_src(seed, 13, 55, 50), 58);

    // boundary checks
    assert_eq!(dst_from_src(seed, 13, 95, 5), 18);

    // some from sample/easy input

    let seed = 79;
    assert_eq!(dst_from_src(seed, 50, 98, 2), 79);
    assert_eq!(dst_from_src(seed, 52, 50, 48), 81);
    let seed = 14;
    assert_eq!(dst_from_src(seed, 50, 98, 2), 14);
    assert_eq!(dst_from_src(seed, 52, 50, 48), 14);
    let seed = 55;
    assert_eq!(dst_from_src(seed, 50, 98, 2), 55);
    assert_eq!(dst_from_src(seed, 52, 50, 48), 57);
    let seed = 13;
    assert_eq!(dst_from_src(seed, 50, 98, 2), 13);
    assert_eq!(dst_from_src(seed, 52, 50, 48), 13);

    let seed = 81;
    assert_eq!(dst_from_src(seed, 0, 15, 37), 81);
    assert_eq!(dst_from_src(seed, 37, 52, 2), 81);
    assert_eq!(dst_from_src(seed, 39, 0, 15), 81);
}

fn parse_seeds(line: &str) -> Vec<u32> {
    let numbers = line // "seeds: 79 14 55 13"
        .split(':') // ["seeds", " 79 14 55 13"]
        .skip(1) // [" 79 14 55 13"]
        .next() // " 79 14 55 13"
        .unwrap()
        .split_whitespace(); // ["79", "14", "55", "13"]

    numbers.fold(vec![], |mut acc, n| {
        acc.push(n.parse().unwrap());
        acc
    })
}

fn main() {
    let lines = read_lines("./easy_input_part_one").unwrap(); // 13
    // let lines = read_lines("./input").unwrap(); // 32609, 14624680
    let mut seeds = vec![];

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("seeds:") {
            seeds = parse_seeds(&line);
        }
    }

    dbg!(seeds);
}
