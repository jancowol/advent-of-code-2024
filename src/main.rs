fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::fs;

    #[test]
    fn day1_1() {
        let data = fs::read_to_string("data/day-1-1.txt").unwrap();
        let mut list1 = vec![];
        let mut list2 = vec![];
        data.split('\n').for_each(|line| {
            let mut parts = line.split_whitespace();
            list1.push(parts.next().unwrap());
            list2.push(parts.next().unwrap());
        });

        // let mut list1 = vec!["3", "4", "2", "1", "3", "3"];
        // let mut list2 = vec!["4", "3", "5", "3", "9", "3"];

        let sum = calc_diffs(&mut list1, &mut list2);
        eprintln!("foo = {:#?}", sum);
        // 2769675
    }

    fn calc_diffs(list1: &mut Vec<&str>, list2: &mut Vec<&str>) -> u32 {
        list1.sort();
        list2.sort();

        let diffs = list1.iter().zip(list2).map(|(sv1, sv2)| {
            let vv1 = sv1.parse::<i32>().unwrap();
            let vv2 = sv2.parse::<i32>().unwrap();
            vv1.abs_diff(vv2)
        });

        diffs.sum()
    }
}
