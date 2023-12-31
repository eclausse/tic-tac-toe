use std::fmt::Display;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    CIRCLE,
    CROSS,
    EMPTY,
}

impl CellState {
    pub fn opposite(&self) -> &Self {
        match self {
            CellState::CIRCLE => {
                return &CellState::CROSS;
            }
            CellState::CROSS => {
                return &CellState::CIRCLE;
            }
            CellState::EMPTY => {
                return &CellState::EMPTY;
            }
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::CIRCLE => write!(f, " O "),
            CellState::CROSS => write!(f, " X "),
            CellState::EMPTY => write!(f, "   "),
        }
    }
}

/* Winning combo in binary */
const WINNING_POSITION: [u16; 8] = [
    0b_111_000_000,
    0b_000_111_000,
    0b_000_000_111,
    0b_100_100_100,
    0b_010_010_010,
    0b_001_001_001,
    0b_100_010_001,
    0b_001_010_100,
];

const TWO_ALIGN_POSITION: [(u16, u16); 24] = [
    /* Bits to match ; Bit the other player must not match */
    (0b_110_000_000, 0b_001_000_000),
    (0b_011_000_000, 0b_100_000_000),
    (0b_101_000_000, 0b_010_000_000),
    (0b_000_011_000, 0b_000_100_000),
    (0b_000_110_000, 0b_000_001_000),
    (0b_000_101_000, 0b_000_010_000),
    (0b_000_000_110, 0b_000_000_001),
    (0b_000_000_011, 0b_000_000_100),
    (0b_000_000_101, 0b_000_000_010),
    (0b_100_100_000, 0b_000_000_100),
    (0b_000_100_100, 0b_100_000_000),
    (0b_100_000_100, 0b_000_100_000),
    (0b_000_010_010, 0b_010_000_000),
    (0b_010_000_010, 0b_000_010_000),
    (0b_010_010_000, 0b_000_000_010),
    (0b_000_001_001, 0b_001_000_000),
    (0b_001_000_001, 0b_000_001_000),
    (0b_001_001_000, 0b_000_000_001),
    (0b_000_010_001, 0b_100_000_000),
    (0b_100_000_001, 0b_000_010_000),
    (0b_100_010_000, 0b_000_000_001),
    (0b_001_010_000, 0b_000_000_100),
    (0b_001_000_100, 0b_000_010_000),
    (0b_000_010_100, 0b_001_000_000),
];

pub struct XPosOPos(pub u16, pub u16);
pub struct Position(pub usize, pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Game {
    pub cell: [[CellState; 3]; 3],
}

impl Game {
    pub fn new() -> Self {
        Game {
            cell: [[CellState::EMPTY; 3]; 3],
        }
    }

    pub fn is_empty_cell(&self, x: usize, y: usize) -> bool {
        if x > 2 || y > 2 {
            panic!("Wrong argument")
        }
        self.cell[x][y] == CellState::EMPTY
    }

    pub fn get_possible_move(&self) -> Vec<Position> {
        let mut res = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                match self.cell[i][j] {
                    CellState::EMPTY => res.push(Position(i, j)),
                    _ => (),
                }
            }
        }
        res
    }

    pub fn get_one_difference(&self, b: &Self) -> Option<Position> {
        for i in 0..3 {
            for j in 0..3 {
                if self.cell[i][j] != b.cell[i][j] {
                    return Some(Position(i, j));
                }
            }
        }
        None
    }

    pub fn set_move(&mut self, pos: &Position, player: CellState) {
        self.cell[pos.0][pos.1] = player;
    }

    pub fn game_to_bits(&self) -> XPosOPos {
        let mut x_bits: u16 = 0;
        let mut o_bits: u16 = 0;

        for i in 0..3 {
            for j in 0..3 {
                match self.cell[i][j] {
                    CellState::CIRCLE => o_bits |= 1 << (i * 3) + j,
                    CellState::CROSS => x_bits |= 1 << (i * 3) + j,
                    _ => (),
                }
            }
        }
        XPosOPos(x_bits, o_bits)
    }

    pub fn is_won(&self) -> Option<CellState> {
        let XPosOPos(x_bits, o_bits) = self.game_to_bits();
        for pos in &WINNING_POSITION {
            if (pos & x_bits) == *pos {
                return Some(CellState::CROSS);
            }
            if (pos & o_bits) == *pos {
                return Some(CellState::CIRCLE);
            }
        }
        None
    }

    pub fn is_over(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.is_empty_cell(i, j) {
                    return false;
                }
            }
        }
        true
    }

    pub fn game_continue(&self) -> bool {
        match self.is_won() {
            Some(i) => match i {
                CellState::CROSS => {
                    print!("You won");
                    return false;
                }
                CellState::CIRCLE => {
                    print!("You lost");
                    return false;
                }
                _ => {
                    panic!("");
                }
            },
            None => {
                if self.is_over() {
                    print!("It's a draw");
                    return false;
                }
                return true;
            }
        }
    }

    pub fn evaluate(&self, player: CellState) -> i32 {
        if player == CellState::EMPTY {
            panic!("Wrong argument: Player empty");
        }

        /* Case three align */
        match self.is_won() {
            Some(x) => {
                if x == player {
                    return 100;
                } else {
                    return -100;
                }
            }
            _ => {}
        }

        let mut score = 0;
        /* Case two align */
        let XPosOPos(x_bits, o_bits) = self.game_to_bits();
        for pos in &TWO_ALIGN_POSITION {
            if player == CellState::CIRCLE && (pos.0 & o_bits) == pos.0 && (pos.1 & x_bits) != pos.1
            {
                score += 10;
            } else if player == CellState::CROSS
                && (pos.0 & x_bits) == pos.0
                && (pos.1 & o_bits) != pos.1
            {
                score -= 10;
            }
        }
        score
    }

    fn get_input() -> Option<u8> {
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .ok()
            .and_then(|_| s.trim().parse::<u8>().ok())
            .filter(|n| *n < 4 && *n > 0)
    }

    pub fn player_move(&mut self) {
        let mut x: u8;
        let mut y: u8;
        let mut valid = false;

        while !valid {
            print!("Please enter x: ");
            match Self::get_input() {
                Some(n) => x = n,
                None => {
                    eprintln!("[Error]: Incorrect input");
                    continue;
                }
            }

            print!("Please enter y: ");
            match Self::get_input() {
                Some(n) => {
                    y = n;
                    valid = true;
                }
                None => {
                    eprintln!("[Error]: Incorrect input");
                    continue;
                }
            }

            if self.is_empty_cell((x - 1).into(), (y - 1).into()) {
                self.cell[(x - 1) as usize][(y - 1) as usize] = CellState::CROSS;
            } else {
                eprintln!("[Error] Cell already taken");
                valid = false;
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            for j in 0..3 {
                write!(f, "{}", self.cell[i][j])?;
                if j < 2 {
                    write!(f, "|")?;
                }
            }
            if i < 2 {
                writeln!(f, "\n-----------")?;
            } else {
                writeln!(f, "")?;
            }
        }
        writeln!(f, "")
    }
}
