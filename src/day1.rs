use std::fs;

pub fn part1() -> u32 {
    let data = fs::read_to_string("data/day-1-1.txt").unwrap();
    let (mut list1, mut list2) = split_lists(&data);
    let sum = calc_diffs(&mut list1, &mut list2);
    sum
}

fn split_lists(data: &String) -> (Vec<&str>, Vec<&str>) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    data.split('\n').for_each(|line| {
        let mut parts = line.split_whitespace();
        list1.push(parts.next().unwrap());
        list2.push(parts.next().unwrap());
    });
    (list1, list2)
}

fn calc_diffs(list1: &mut Vec<&str>, list2: &mut Vec<&str>) -> u32 {
    list1.sort();
    list2.sort();

    let diffs = list1.iter().zip(list2).map(|(sv1, sv2)| {
        let v1 = sv1.parse::<i32>().unwrap();
        let v2 = sv2.parse::<i32>().unwrap();
        v1.abs_diff(v2)
    });

    diffs.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day1::calc_diffs;
    use itertools::Itertools;
    use std::ops::Mul;

    #[test]
    fn sums_all_diffs() {
        let mut list1 = vec!["3", "4", "2", "1", "3", "3"];
        let mut list2 = vec!["4", "3", "5", "3", "9", "3"];
        let sum = calc_diffs(&mut list1, &mut list2);
        assert_eq!(sum, 11);
    }

    #[test]
    fn similarity_score() {
        let mut list1 = vec!["3", "4", "2", "1", "3", "3"];
        let mut list2 = vec!["4", "3", "5", "3", "9", "3"];

        let l1 = list1
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec();

        let l2 = list2
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec();

        let foo: i32 = l1
            .iter()
            .map(|value| {
                let count = l2.iter().filter(|x| value.eq(x)).count() as i32;
                *value * count
            })
            .sum();

        assert_eq!(foo, 31);
    }
}
