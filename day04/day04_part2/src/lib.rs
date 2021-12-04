#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct BingoNumber {
    val: u64,
    hit: bool,
}

impl BingoNumber {
    fn hit(&mut self) {
        self.hit = true;
    }

    fn is_hit(&self) -> bool {
        self.hit
    }

    fn val(&self) -> u64 {
        self.val
    }
}

impl From<&u64> for BingoNumber {
    fn from(val: &u64) -> Self {
        Self {
            val: *val,
            hit: false,
        }
    }
}

impl From<&BingoNumber> for u64 {
    fn from(b: &BingoNumber) -> u64 {
        b.val()
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct BingoRow {
    numbers: [BingoNumber; 5],
}

impl From<[u64; 5]> for BingoRow {
    fn from(vals: [u64; 5]) -> Self {
        let mut numbers = [BingoNumber::default(); 5];
        for (ii, nn) in vals.iter().enumerate() {
            numbers[ii] = nn.into();
        }
        Self { numbers }
    }
}

impl BingoRow {
    fn is_full(&self) -> bool {
        self.numbers.iter().all(|x| x.is_hit())
    }

    fn hit_for_number(&mut self, x: u64) {
        self.numbers
            .iter_mut()
            .filter(|y| y.val() == x)
            .for_each(|x| x.hit());
    }

    fn get_numbers_not_hit_sum(&self) -> u64 {
        self.numbers
            .iter()
            .filter(|n| !n.is_hit())
            .map(|x| x.val())
            .sum()
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct BingoBoard {
    rows: [BingoRow; 10],
}

impl BingoBoard {
    fn new() -> Self {
        Self {
            rows: Default::default(),
        }
    }

    fn hit_for_number(&mut self, x: u64) {
        self.rows.iter_mut().for_each(|r| r.hit_for_number(x));
    }

    fn has_full_row(&self) -> bool {
        self.rows.iter().any(BingoRow::is_full)
    }

    fn get_numbers_not_hit_sum(&self) -> u64 {
        self.rows
            .iter()
            .take(5)
            .map(BingoRow::get_numbers_not_hit_sum)
            .sum()
    }
}

impl From<[&str; 5]> for BingoBoard {
    fn from(strings: [&str; 5]) -> Self {
        let mut board = Self::new();
        // Add rows
        for (i, ss) in strings.iter().enumerate() {
            let mut row = [0u64; 5];
            for (ii, s) in ss.split(' ').filter(|s| *s != "").enumerate() {
                row[ii] = s.parse().unwrap();
            }
            board.rows[i] = row.into();
        }
        // Add columns
        for x in 0..5 {
            let mut row = [0u64; 5];
            for (ii, ss) in strings.iter().enumerate() {
                let num = ss
                    .split(' ')
                    .filter(|s| *s != "")
                    .map(|s| s.parse().unwrap())
                    .nth(x)
                    .unwrap();
                row[ii] = num;
            }
            board.rows[x + 5] = row.into();
        }
        board
    }
}

impl BingoBoard {}

pub fn solve(contents: &str) -> u64 {
    let mut split = contents.split('\n');
    let number_str = split.next().unwrap();
    let mut input_numbers = [0u64; 100];
    for (ii, ss) in number_str.split(',').enumerate() {
        input_numbers[ii] = ss.parse().unwrap()
    }

    split.next(); // skip first empty line

    let mut boards = Vec::with_capacity(100);
    let mut curr_board = [""; 5];
    let mut count = 0;
    for line in split {
        if line == "" {
            let board: BingoBoard = curr_board.into();
            boards.push(board);
            curr_board = [""; 5];
            count = 0;
        } else {
            curr_board[count] = line;
            count += 1;
        }
    }

    let mut last_score = 0;
    let mut boards_won: [bool; 100] = [false; 100];
    for num in input_numbers {
        for (ii, board) in &mut boards.iter_mut().enumerate() {
            if boards_won[ii] {
                continue;
            }
            board.hit_for_number(num);
            if board.has_full_row() {
                let numbers_not_hit: u64 = board.get_numbers_not_hit_sum();
                last_score = numbers_not_hit * num;
                boards_won[ii] = true
            }
        }
    }

    return last_score;
}

#[cfg(test)]
mod test {
    use std::io::Read;
    const INPUT_FN: &str = "input";
    #[test]
    fn test_solve() {
        let mut fd = std::fs::File::open(INPUT_FN).unwrap();
        let mut contents = String::new();
        fd.read_to_string(&mut contents).unwrap();
        let last_score = crate::solve(&contents);
        assert_eq!(12738, last_score);
    }
}
