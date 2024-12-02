fn check_safety<'a>(report: impl Iterator<Item = &'a i32>) -> bool {
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

struct ProblemDampener<T> {
    report: T,
    skip: usize,
}

impl<T> ProblemDampener<T> {
    fn new(report: T, skip: usize) -> Self {
        Self { report, skip }
    }
}

impl<T> Iterator for ProblemDampener<T>
where
    T: Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip == 0 {
            self.report.next();
            self.skip = std::usize::MAX
        }
        self.skip = self.skip - 1;
        self.report.next()
    }
}

pub fn day2(input: &str) {
    let mut safe = 0;
    let mut safe2 = 0;

    let mut report: Vec<i32> = Vec::new();
    for line in input.lines() {
        report.clear();
        for a in line
            .split_ascii_whitespace()
            .map(|token| token.parse().unwrap())
        {
            report.push(a);
        }

        if check_safety(report.iter()) {
            safe = safe + 1;
            safe2 = safe2 + 1;
        } else {
            for i in 0..report.len() {
                if check_safety(ProblemDampener::new(report.iter(), i)) {
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
