fn check_safety<T: Iterator<Item = i32>>(report: T) -> bool {
    let mut ascending = None;
    let mut last = None;
    for value in report {
        if let Some(last) = last {
            match ascending {
                None => {
                    if value > last {
                        ascending = Some(true);
                    } else if value < last {
                        ascending = Some(false);
                    } else {
                        return false;
                    }
                }
                Some(true) => {
                    if value < last {
                        return false;
                    }
                }
                Some(false) => {
                    if value > last {
                        return false;
                    }
                }
            }
            if (last - value).abs() < 1 || (last - value).abs() > 3 {
                return false;
            }
        }
        last = Some(value);
    }

    true
}

struct ProblemDampener {
    report: Vec<i32>,
    idx: usize,
    skip: usize,
}

impl ProblemDampener {
    fn new(report: &Vec<i32>, skip: usize) -> Self {
        Self {
            report: report.clone(),
            idx: 0,
            skip,
        }
    }
}

impl Iterator for ProblemDampener {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.skip {
            self.idx = self.idx + 1
        }
        if self.idx >= self.report.len() {
            None
        } else {
            let i = self.idx;
            self.idx = self.idx + 1;
            Some(self.report[i])
        }
    }
}

pub fn day2(input: &str) {
    let mut safe = 0;
    let mut safe2 = 0;
    for line in input.lines() {
        let report: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|token| token.parse().unwrap())
            .collect();

        if check_safety(report.clone().into_iter()) {
            safe = safe + 1;
            safe2 = safe2 + 1;
        } else {
            for i in 0..report.len() {
                if check_safety(ProblemDampener::new(&report, i)) {
                    safe2 = safe2 + 1;
                    break;
                }
            }
        }
    }

    println!("safe: {safe}");
    println!("safe2: {safe2}");
}

#[test]
fn test0() {
    day2(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    );
}
