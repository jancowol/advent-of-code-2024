use itertools::{FoldWhile, Itertools};
use std::fs;

pub(crate) fn part1() -> usize {
    let reports = read_reports();
    let safe_count = reports.into_iter().filter(|r| is_safe(r.to_vec())).count();
    safe_count
}

// pub(crate) fn part2() -> usize {
//     count_safe_reports()
// }

fn read_reports() -> Vec<Vec<i32>> {
    let data = fs::read_to_string("data/day-2-1.txt").unwrap();
    data.split('\n')
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn is_safe(report: Vec<i32>) -> bool {
    let diffs = report.windows(2).map(|w| w[0] - w[1]);
    diff_seq_is_safe(diffs)
}

fn safe_with_dampener(report: Vec<i32>) -> bool {
    let mut dampened_reports = report
        .iter()
        .enumerate()
        .map(|(i, val)| report.iter().take(i + 1).chain(report.iter().skip(i + 2)));
    for dampened_report in dampened_reports {
        let report_values = dampened_report.collect_vec();
        let diffs = report_values.windows(2).map(|w| w[0] - w[1]);
        if diff_seq_is_safe(diffs) {
            return true;
        }
    }
    false
}

fn diff_seq_is_safe(mut diffs: impl Iterator<Item = i32>) -> bool {
    let all_safe = diffs.fold_while(None, |prev: Option<i32>, next: i32| {
        let diff = next.abs();
        let prev_signum = match prev {
            None => next.signum(),
            Some(prev) => prev.signum(),
        };
        if next.signum() == prev_signum && diff >= 1 && diff <= 3 {
            FoldWhile::Continue(Some(next))
        } else {
            FoldWhile::Done(None)
        }
    });
    match all_safe {
        FoldWhile::Continue(_) => true,
        FoldWhile::Done(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools;

    #[test]
    fn test_levels() {
        let input = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

        let reports = read_reports();
        let safe_count = reports.into_iter().filter(|r| is_safe(r.to_vec())).count();
        assert_eq!(safe_count, 2);
    }

    #[test]
    fn is_safe_tests() {
        let safe1: Vec<i32> = vec![7, 6, 4, 2, 1];
        assert!(is_safe(safe1));

        let safe2 = vec![1, 3, 6, 7, 9];
        assert!(is_safe(safe2));

        let unsafe1 = vec![1, 2, 7, 8, 9];
        assert!(!is_safe(unsafe1));

        let unsafe2 = vec![9, 7, 6, 2, 1];
        assert!(!is_safe(unsafe2));

        let unsafe3 = vec![1, 3, 2, 4, 5];
        assert!(!is_safe(unsafe3));

        let unsafe4 = vec![8, 6, 4, 4, 1];
        assert!(!is_safe(unsafe4));

        let unsafe5 = vec![16, 17, 18, 21, 23, 24, 27, 24];
        assert!(!is_safe(unsafe5));

        let unsafe6 = vec![74, 76, 79, 81, 82, 85, 85];
        assert!(!is_safe(unsafe6));

        let unsafe7 = vec![48, 51, 53, 54, 55, 59];
        assert!(!is_safe(unsafe7));

        let unsafe8 = vec![9, 12, 9, 11, 14, 16, 17, 20];
        assert!(!is_safe(unsafe8));

        let unsafe9 = vec![6, 6, 4, 3, 2, 1];
        assert!(!is_safe(unsafe9));

        let unsafe10 = vec![8, 8, 6, 4, 1];
        assert!(!is_safe(unsafe10));

        let unsafe11 = vec![1, 4, 6, 8, 8];
        assert!(!is_safe(unsafe11));
    }

    #[test]
    fn is_safe_with_dampener() {
        let safe1 = vec![1, 3, 2, 4, 5];
        assert!(safe_with_dampener(safe1));

        let safe2 = vec![8, 6, 4, 4, 1];
        assert!(safe_with_dampener(safe2));

        let unsafe1 = vec![1, 2, 7, 8, 9];
        assert!(!safe_with_dampener(unsafe1));
    }
}
