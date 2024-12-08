use itertools::Itertools;
use std::fs;
use std::ops::{Add, Sub};

pub(crate) fn part1() -> i32 {
    let raw_bytes = fs::read("data/day-4-1.txt").unwrap();
    let mut data = bytes_to_grid(&raw_bytes, 140);
    let count = calc(&mut data);
    // 2575
    count
}

pub(crate) fn part2() -> i32 {
    let raw_bytes = fs::read("data/day-4-1.txt").unwrap();
    let count = find_xmas(&raw_bytes, 140);
    count
}

fn bytes_to_grid(raw_bytes: &Vec<u8>, row_len: usize) -> Vec<&[u8]> {
    let mut data = Vec::with_capacity(row_len);
    let mut start: usize = 0;
    for i in 0..row_len {
        let end = start + row_len;
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

fn find_xmas(raw_bytes: &Vec<u8>, i1: usize) -> i32 {
    let mut data = bytes_to_grid(&raw_bytes, i1);
    let row_count = data.len();
    let col_count = data[0].len();

    let locations = (1..data.len() as i64 - 1)
        .cartesian_product(1..(i1.sub(1) as i64))
        .collect_vec();

    let mut count = 0;
    const MAS: &'static [u8; 3] = b"MAS";
    const SAM: &'static [u8; 3] = b"SAM";

    for (y, x) in locations {
        let char = data[y as usize][x as usize];
        if char == b'A' {
            // found a potential cross, get its words
            let l2r_diag_chars = top_left_to_bottom_right(y, x);
            let l2r_word = get_word(&data, l2r_diag_chars.collect_vec());

            let r2l_diag_chars = top_right_to_bottom_left(y, x);
            let r2l_word = get_word(&data, r2l_diag_chars.collect_vec());

            if (l2r_word == MAS || l2r_word == SAM) && (r2l_word == MAS || r2l_word == SAM) {
                count += 1;
            }
        }
    }
    count
}

fn top_right_to_bottom_left(y: i64, x: i64) -> impl Iterator<Item = (usize, usize)> {
    (-1i64..=1).map(move |i| (y.add(i) as usize, x.add(i * -1) as usize))
}

fn top_left_to_bottom_right(y: i64, x: i64) -> impl Iterator<Item = (usize, usize)> {
    (-1i64..=1).map(move |i| (y.add(i) as usize, x.add(i) as usize))
}

fn get_word(data: &Vec<&[u8]>, word_char_locs: Vec<(usize, usize)>) -> Vec<u8> {
    word_char_locs
        .iter()
        .map(|(y, x)| data[*y][*x])
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let raw_bytes = fs::read("data/day-4-1.txt").unwrap();
        let mut data = bytes_to_grid(&raw_bytes, 140);
        let count = calc(&mut data);
        assert_eq!(count, 2575);
    }

    #[test]
    fn test_part2() {
        let test_input = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .trim();

        const SIZE: usize = 10;

        let raw_bytes = test_input.as_bytes().to_vec();
        let count = find_xmas(&raw_bytes, SIZE);
        assert_eq!(count, 9);
    }
}
