use itertools::Itertools;
use std::fs;

pub fn part1() -> u32 {
    let (mut list1, mut list2) = read_into_ids();
    calc_diffs(&mut list1, &mut list2)
}

pub fn part2() -> i32 {
    let (list1, list2) = read_into_ids();
    calc_similarity_score(list1, list2)
}

fn read_into_ids() -> (Vec<i32>, Vec<i32>) {
    let data = fs::read_to_string("data/day-1-1.txt").unwrap();
    let (list1, list2) = split_lists(&data);
    (to_i32_vec(list1), to_i32_vec(list2))
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

fn calc_diffs(list1: &mut Vec<i32>, list2: &mut Vec<i32>) -> u32 {
    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2)
        .map(|(sv1, sv2)| sv1.abs_diff(*sv2))
        .sum()
}

fn to_i32_vec(vec: Vec<&str>) -> Vec<i32> {
    vec.iter().map(|s| s.parse::<i32>().unwrap()).collect_vec()
}

fn calc_similarity_score(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    list1
        .iter()
        .map(|v1| {
            let count = list2.iter().filter(|v2| v1.eq(v2)).count() as i32;
            *v1 * count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_all_diffs() {
        let list1 = vec!["3", "4", "2", "1", "3", "3"];
        let list2 = vec!["4", "3", "5", "3", "9", "3"];
        let mut l1 = to_i32_vec(list1);
        let mut l2 = to_i32_vec(list2);

        let sum = calc_diffs(&mut l1, &mut l2);
        assert_eq!(sum, 11);
    }

    #[test]
    fn similarity_score() {
        let list1 = vec!["3", "4", "2", "1", "3", "3"];
        let list2 = vec!["4", "3", "5", "3", "9", "3"];
        let l1 = to_i32_vec(list1);
        let l2 = to_i32_vec(list2);

        let similarity_score = calc_similarity_score(l1, l2);
        assert_eq!(similarity_score, 31);
    }
}
