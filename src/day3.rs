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
