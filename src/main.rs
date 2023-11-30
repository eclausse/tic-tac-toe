use std::fmt::Display;
use std::io::{stdin, stdout, Write};

mod game;
mod node;

use crate::game::*;
use crate::node::*;

#[derive(Debug)]
struct Tree(Box<Node>);

impl Tree {
    pub fn new(game: Game, maximizing_player: CellState) -> Self {
        if maximizing_player == CellState::EMPTY {
            panic!("Wrong argument");
        }
        Tree(Box::new(Node {
            game,
            utility: 0,
            depth: 0,
            maximizing_player,
            childs: Vec::new(),
        }))
    }

    pub fn generate_min_max(&mut self) {
        self.0.generate_min_max();
    }

    pub fn get_move(&self) -> Option<Position> {
        if self.0.depth % 2 == 0 {
            return None;
        }
        let best_node: &Node = self.0.childs.iter().max_by_key(|c| c.as_ref().utility).unwrap();

        self.0.game.get_one_difference(&best_node.game)
    }

    pub fn set_move(&mut self, pos: Position) {
        if self.0.depth % 2 != 0 {
            return;
        }
        /* Create the new game */
        let mut g = self.0.game.clone();
        g.set_move(pos, self.0.maximizing_player);

        /* Find the new game node */
        self.0.childs.retain(|e| e.game == g);
        self.0 = self.0.childs.pop().unwrap();
    }
}

fn main() {
    let mut g = Game::new();

    let mut t = Tree::new(g, CellState::CIRCLE);
    t.generate_min_max();
    let p = t.get_move().unwrap();
    g.set_move(p, CellState::CIRCLE);
    print!("{g}");
    //print!("{:#?}", t);
}
