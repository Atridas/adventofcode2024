struct Words {
    words: Vec<u8>,
    width: i32,
    height: i32,
}

impl Words {
    fn new(input: &str) -> Self {
        let mut words = Vec::new();
        let width = input.lines().next().unwrap().len() as i32;
        let mut height = 0;
        for line in input.lines() {
            for c in line.as_bytes() {
                words.push(*c);
            }
            height = height + 1;
        }
        Self {
            words,
            width,
            height,
        }
    }

    fn get_at(&self, i: i32, j: i32) -> u8 {
        if i < 0 || j < 0 || i >= self.width || j >= self.height {
            b'\0'
        } else {
            self.words[(j * self.width + i) as usize]
        }
    }

    fn find_xmas(&self, i: i32, j: i32, di: i32, dj: i32) -> bool {
        self.get_at(i, j) == b'X'
            && self.get_at(i + di, j + dj) == b'M'
            && self.get_at(i + di * 2, j + dj * 2) == b'A'
            && self.get_at(i + di * 3, j + dj * 3) == b'S'
    }

    fn count_xmas(&self, i: i32, j: i32) -> i32 {
        let mut result = 0;
        if self.find_xmas(i, j, 1, 0) {
            result = result + 1;
        }
        if self.find_xmas(i, j, -1, 0) {
            result = result + 1;
        }
        if self.find_xmas(i, j, 0, 1) {
            result = result + 1;
        }
        if self.find_xmas(i, j, 0, -1) {
            result = result + 1;
        }
        if self.find_xmas(i, j, 1, 1) {
            result = result + 1;
        }
        if self.find_xmas(i, j, 1, -1) {
            result = result + 1;
        }
        if self.find_xmas(i, j, -1, 1) {
            result = result + 1;
        }
        if self.find_xmas(i, j, -1, -1) {
            result = result + 1;
        }
        result
    }

    fn count_all_xmas(&self) -> i32 {
        let mut result = 0;
        for i in 0..self.width {
            for j in 0..self.height {
                result = result + self.count_xmas(i, j);
            }
        }
        result
    }

    fn find_x_mas(&self, i: i32, j: i32) -> bool {
        if self.get_at(i, j) != b'A' {
            return false;
        }
        let c1 = self.get_at(i - 1, j - 1);
        let c2 = self.get_at(i + 1, j + 1);
        let (d1, d2) = if c1 < c2 { (c1, c2) } else { (c2, c1) };
        let c3 = self.get_at(i - 1, j + 1);
        let c4 = self.get_at(i + 1, j - 1);
        let (d3, d4) = if c3 < c4 { (c3, c4) } else { (c4, c3) };

        d1 == b'M' && d2 == b'S' && d3 == b'M' && d4 == b'S'
    }

    fn count_all_x_mas(&self) -> i32 {
        let mut result = 0;
        for i in 0..self.width {
            for j in 0..self.height {
                if self.find_x_mas(i, j) {
                    result = result + 1;
                }
            }
        }
        result
    }
}

pub fn day4(input: &str) {
    let words = Words::new(input);
    println!("xmas: {}", words.count_all_xmas());
    println!("x_mas: {}", words.count_all_x_mas());
}

#[test]
fn test0() {
    day4(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    );
}
