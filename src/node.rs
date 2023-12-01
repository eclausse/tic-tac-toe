use std::fmt::Display;

use crate::game::*;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub game: Game,
    pub utility: i32,
    pub depth: u32,
    pub maximizing_player: CellState,
    pub childs: Vec<Box<Node>>,
}

impl Node {
    pub fn new(game: Game, depth: u32, maximizing_player: CellState) -> Self {
        Node {
            game,
            utility: if depth % 2 == 0 { i32::MIN } else { i32::MAX },
            depth,
            maximizing_player,
            childs: Vec::new(),
        }
    }

    pub fn populate(&mut self) {
        if self.game.is_won().is_some() {
            return;
        }

        let possible_moves = self.game.get_possible_move();

        for m in &possible_moves {
            let mut g = self.game.clone();
            if self.depth % 2 == 0 {
                g.cell[m.0][m.1] = self.maximizing_player;
            } else {
                g.cell[m.0][m.1] = *self.maximizing_player.opposite();
            }
            let n = Node::new(g.to_owned(), self.depth + 1, self.maximizing_player);
            self.childs.push(Box::new(n));
        }
    }

    pub fn generate_min_max(&mut self) {
        self.populate();
        for c in self.childs.iter_mut().map(|b| b.as_mut()) {
            c.generate_min_max();
        }
        if self.childs.is_empty() {
            if self.depth % 2 == 0 {
                self.utility = -self.game.evaluate(*self.maximizing_player.opposite());
            } else {
                self.utility = self.game.evaluate(self.maximizing_player);
            }
        } else {
            if self.depth % 2 == 0 {
                self.utility = self.childs.iter().map(|c| c.utility).max().unwrap();
            } else {
                self.utility = self.childs.iter().map(|c| c.utility).min().unwrap();
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
