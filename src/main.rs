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

    pub fn get_move(&mut self) -> Option<Position> {
        if self.0.depth % 2 != 0 {
            return None;
        }
        let best_node: &Node = self
            .0
            .childs
            .iter()
            .max_by_key(|c| c.as_ref().utility)
            .unwrap();

        self.0.game.get_one_difference(&best_node.game)
    }

    pub fn set_move(&mut self, pos: Position, player: CellState) {
        /* Create the new game */
        let mut g = self.0.game.clone();
        g.set_move(&pos, player);

        /* Find the new game node */
        self.0.childs.retain(|e| e.game == g);
        self.0 = self.0.childs.pop().unwrap();
    }
}

fn main() {
    /* Create a game */
    let mut board = Game::new();

    let mut position_played;
    let mut prev_game: Game = board;

    /* Player first move */
    board.player_move();
    position_played = prev_game.get_one_difference(&board).unwrap();
    board.set_move(&position_played, CellState::CROSS);

    /* Create min max */
    let mut possibility_tree = Tree::new(board.clone(), CellState::CIRCLE);
    possibility_tree.generate_min_max();

    while board.game_continue() {
        position_played = possibility_tree.get_move().unwrap();
        board.set_move(&position_played, CellState::CIRCLE);
        possibility_tree.set_move(position_played, CellState::CIRCLE);

        print!("{board}");

        if !board.game_continue() {
            break;
        }

        prev_game = board.clone();
        board.player_move();
        position_played = prev_game.get_one_difference(&board).unwrap();
        board.set_move(&position_played, CellState::CROSS);
        possibility_tree.set_move(position_played, CellState::CROSS);
    }
}

/*
g.cell[0][0] = CellState::CROSS;
    g.cell[1][0] = CellState::CROSS;
    g.cell[2][0] = CellState::CROSS;
    g.cell[2][2] = CellState::CROSS;

    g.cell[2][0] = CellState::CIRCLE;
    g.cell[1][1] = CellState::CIRCLE;
    g.cell[1][2] = CellState::CIRCLE;

    print!("{g}");

    p = t.get_move().unwrap();
    g.set_move(&p, CellState::CIRCLE);
    t.set_move(p, CellState::CIRCLE);

    print!("{g}");

    print!("{:#?}", t);
*/
