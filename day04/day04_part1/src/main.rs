use std::io::Read;
const INPUT_FN: &str = "input";

#[derive(Debug)]
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

#[derive(Default, Debug)]
struct BingoRow {
    numbers: Vec<BingoNumber>,
}

impl From<Vec<u64>> for BingoRow {
    fn from(vals: Vec<u64>) -> Self {
        let numbers = vals.iter().map(|v| v.into()).collect();
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

    fn get_numbers_not_hit(&self) -> Vec<u64> {
        self.numbers
            .iter()
            .filter(|n| !n.is_hit())
            .map(|n| n.into())
            .collect()
    }
}

#[derive(Default, Debug)]
struct BingoBoard {
    rows: [BingoRow; 10],
}

impl BingoBoard {
    fn hit_for_number(&mut self, x: u64) {
        self.rows.iter_mut().for_each(|r| r.hit_for_number(x));
    }

    fn has_full_row(&self) -> bool {
        self.rows.iter().any(BingoRow::is_full)
    }

    fn get_numbers_not_hit(&self) -> Vec<u64> {
        self.rows
            .iter()
            .take(5)
            .flat_map(BingoRow::get_numbers_not_hit)
            .collect()
    }
}

impl From<[&str; 5]> for BingoBoard {
    fn from(strings: [&str; 5]) -> Self {
        let mut board = Self::default();
        // Add rows
        for (i, ss) in strings.iter().enumerate() {
            let split: BingoRow = ss
                .split(' ')
                .filter(|s| *s != "")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>()
                .into();
            board.rows[i] = split;
        }
        // Add columns
        for x in 0..5 {
            let mut col = Vec::new();
            for ss in strings {
                let num = ss
                    .split(' ')
                    .filter(|s| *s != "")
                    .map(|s| s.parse().unwrap())
                    .nth(x)
                    .unwrap();
                col.push(num);
            }
            board.rows[x + 5] = col.into();
        }
        board
    }
}

impl BingoBoard {}

fn main() {
    let mut fd = std::fs::File::open(INPUT_FN).unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();

    let mut split = contents.split('\n');
    let numbersStr = split.next().unwrap();
    let input_numbers = numbersStr
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    //println!("{:?}", input_numbers);

    split.next(); // skip first empty line

    let board_lines = split.collect::<Vec<&str>>();
    let mut boards = Vec::new();
    for ri in (0..board_lines.len()).step_by(6) {
        let board: [&str; 5] = board_lines[ri..ri + 5].try_into().unwrap();
        let board: BingoBoard = board.into();
        boards.push(board);
    }

    let mut got_hit = false;
    for num in input_numbers {
        for board in &mut boards {
            board.hit_for_number(num);
            if board.has_full_row() {
                got_hit = true;
                let numbers_not_hit: u64 = board.get_numbers_not_hit().iter().sum();
                println!("{:#?}", numbers_not_hit * num);
                break;
            }
        }
        if got_hit {
            break;
        }
    }
}
