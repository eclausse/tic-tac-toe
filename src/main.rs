use std::fmt::Display;
use std::io::{stdin, stdout, Write};

#[derive(Clone, Copy, PartialEq)]
enum CellState {
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

const TWO_ALIGN_POSITION: [u16; 24] = [
    0b_110_000_000,
    0b_011_000_000,
    0b_101_000_000,
    0b_000_011_000,
    0b_000_110_000,
    0b_000_101_000,
    0b_000_000_110,
    0b_000_000_011,
    0b_000_000_101,
    0b_100_100_000,
    0b_000_100_100,
    0b_100_000_100,
    0b_000_010_010,
    0b_010_000_010,
    0b_010_010_000,
    0b_000_001_001,
    0b_001_000_001,
    0b_001_001_000,
    0b_000_010_001,
    0b_100_000_001,
    0b_100_010_000,
    0b_001_010_000,
    0b_001_000_100,
    0b_000_010_100,
];

struct XPosOPos(u16, u16);

#[derive(Clone, Copy)]
struct Game {
    cell: [[CellState; 3]; 3],
}

impl Game {
    pub fn new() -> Self {
        Game {
            cell: [[CellState::EMPTY; 3]; 3],
        }
    }

    pub fn is_empty_cell(&self, x: u8, y: u8) -> bool {
        if x > 2 || y > 2 {
            panic!("Wrong argument")
        }
        self.cell[x as usize][y as usize] != CellState::EMPTY
    }

    pub fn game_to_bits(&self) -> XPosOPos {
        let mut x_bits: u16 = 0;
        let mut o_bits: u16 = 0;

        for i in 0..3 {
            for j in 0..3 {
                match self.cell[i][j] {
                    CellState::CIRCLE => o_bits |= 1 << (i * 3) + (j + 1),
                    CellState::CROSS => x_bits |= 1 << (i * 3) + (j + 1),
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

    pub fn evaluate(&self, player: CellState) -> u32 {
        if player == CellState::EMPTY {
            panic!("Wrong argument: Player empty");
        }

        /* Case three align */
        match self.is_won() {
            Some(x) => {
                if x == player {
                    return 1000;
                }
            }
            _ => {}
        }

        let mut score = 0;

        /* Case two align */
        let XPosOPos(x_bits, o_bits) = self.game_to_bits();
        for pos in &TWO_ALIGN_POSITION {
            if player == CellState::CROSS && (pos & x_bits) == *pos {
                score += 10;
            }
            if player == CellState::CIRCLE && (pos & o_bits) == *pos {
                score += 10;
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
        let mut x: u8 = 0;
        let mut y: u8 = 0;
        let mut valid = false;

        while !valid {
            print!("Please enter x: ");
            match Self::get_input()
            {
                Some(n) => x = n,
                None => {
                    eprintln!("[Error]: Incorrect input");
                    continue;
                }
            }

            print!("Please enter y: ");
            match Self::get_input()
            {
                Some(n) => {
                    y = n;
                    valid = true;
                }
                None => {
                    eprintln!("[Error]: Incorrect input");
                    continue;
                }
            }

            if !self.is_empty_cell(x - 1, y - 1) {
                self.cell[(y - 1) as usize][(x - 1) as usize] = CellState::CROSS;
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

struct Node {
    game: Game,
    utility: u32,
    depth: u32,
    maximizing_player: CellState,
    childs: Vec<Box<Node>>,
}

impl Node {
    pub fn new(game: Game, depth: u32, maximizing_player: CellState) -> Self {
        Node {
            game,
            utility: game.evaluate(if depth % 2 == 0 {
                maximizing_player
            } else {
                *maximizing_player.opposite()
            }),
            depth,
            maximizing_player,
            childs: Vec::new(),
        }
    }
}

struct Tree(Node);

impl Tree {
    pub fn new(game: Game, maximizing_player: CellState) -> Self {
        if maximizing_player == CellState::EMPTY {
            panic!("Wrong argument");
        }
        Tree(Node {
            game,
            utility: 0,
            depth: 0,
            maximizing_player,
            childs: Vec::new(),
        })
    }
}

fn main() {
    let mut game = Game::new();
    game.player_move();
    game.player_move();
    game.player_move();

    print!("{game}");
    match game.is_won() {
        Some(p) => println!("{p}"),
        None => eprintln!("No winner"),
    }
}
