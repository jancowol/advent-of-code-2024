use itertools::Itertools;
use std::fs;

pub(crate) fn part1() -> i32 {
    let raw_bytes = fs::read("data/day-4-1.txt").unwrap();
    let mut data = bytes_to_grid(&raw_bytes);
    let count = calc(&mut data);
    // 2575
    count
}

fn bytes_to_grid(raw_bytes: &Vec<u8>) -> Vec<&[u8]> {
    let mut data = Vec::with_capacity(140);
    let mut start: usize = 0;
    for i in 0..140 {
        let end = start + 140;
        let slice = &raw_bytes[start..end];
        data.push(slice);
        start = end + 1;
    }
    data
}

fn word_is_xmas(word: &[u8]) -> bool {
    word == b"XMAS"
}

fn calc(data: &Vec<&[u8]>) -> i32 {
    let mut count = 0;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            // right
            let look_right = x <= data[y].len() - 4;
            let look_left = x >= 3;
            let look_down = y <= data.len() - 4;
            let look_up = y >= 3;

            if look_right {
                let word = &data[y][x..x + 4];
                if word_is_xmas(word) {
                    count += 1
                }
            }
            // left
            if look_left {
                let mut word = &mut data[y][x - 3..x + 1].to_vec();
                word.reverse();
                if word_is_xmas(word) {
                    count += 1;
                }
            }
            // down
            if look_down {
                let word = &data[y..y + 4].iter().map(|line| line[x]).collect_vec();
                if word_is_xmas(word) {
                    count += 1;
                }
            }
            // up
            if look_up {
                let mut word = data[y - 3..y + 1].iter().map(|line| line[x]).collect_vec();
                word.reverse();
                if word_is_xmas(&word) {
                    count += 1;
                }
            }

            // diagonal right/down
            if look_down && look_right {
                let lines = &data[y..y + 4];
                let word = lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| line[x + i])
                    .collect_vec();
                if word_is_xmas(&word) {
                    count += 1;
                }
            }

            // diagonal left/down
            if look_down && look_left {
                let lines = &data[y..y + 4];
                let word = lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| line[x - i])
                    .collect_vec();
                if word_is_xmas(&word) {
                    count += 1;
                }
            }

            // diagonal left/up
            if look_up && look_left {
                let lines = &data[y - 3..y + 1];
                let word = lines
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, line)| line[x - i])
                    .collect_vec();
                if word_is_xmas(&word) {
                    count += 1;
                }
            }

            // diagonal right/up
            if look_up && look_right {
                let lines = &data[y - 3..y + 1];
                let word = lines
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, line)| line[x + i])
                    .collect_vec();
                if word_is_xmas(&word) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let raw_bytes = fs::read("data/day-4-1.txt").unwrap();
        let mut data = bytes_to_grid(&raw_bytes);
        let count = calc(&mut data);
        assert_eq!(count, 2575);
    }
}
