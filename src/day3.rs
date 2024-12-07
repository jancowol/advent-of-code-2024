use itertools::Itertools;
use regex::Regex;
use std::fs;

pub(crate) fn part1() -> i32 {
    let data = fs::read_to_string("data/day-3-1.txt").unwrap();
    let re = Regex::new(r"mul\((\d|\d{2}|\d{3}),(\d|\d{2}|\d{3})\)").unwrap();
    let total: i32 = re
        .captures_iter(data.as_str())
        .map(|cap| {
            let pair = (
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            );
            let prod = pair.0 * pair.1;
            prod
        })
        .sum();
    total
}

pub(crate) fn part2() -> i32 {
    let data = fs::read_to_string("data/day-3-1.txt").unwrap();
    let re = Regex::new(r"(don't|do)|(mul\((\d|\d{2}|\d{3}),(\d|\d{2}|\d{3})\))").unwrap();
    let mut total: i32 = 0;
    let mut active = true;
    for cap in re.captures_iter(data.as_str()) {
        let x = cap.get(0).unwrap().as_str();
        if x == "do" {
            active = true;
        } else if x == "don't" {
            active = false;
        } else {
            if active {
                let pair = (
                    cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                );
                let prod = pair.0 * pair.1;
                total += prod;
            }
        }
    }
    // 83158140
    total
}
