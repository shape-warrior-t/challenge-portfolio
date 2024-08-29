//! Module for the game's rules and state.

use crate::bloxorz_model::block::{Block, Direction};
use crate::bloxorz_model::board::{Board, Tile};

/// A game of Bloxorz in a specific state.
#[derive(Clone, Copy)]
pub struct Game<'a> {
    board: &'a Board,
    block: Block,
}

/// Information about the final outcome of a game of Bloxorz.
#[derive(Clone, Copy)]
pub enum Status<'a> {
    /// The player successfully completed the stage.
    Win,
    /// The player entered a fail state.
    Loss,
    /// The game is still ongoing.
    Active(ActiveGame<'a>),
}

/// An ongoing game of Bloxorz in which the player can still make moves.
#[derive(Clone, Copy)]
pub struct ActiveGame<'a> {
    board: &'a Board,
    block: Block,
}

impl<'a> Game<'a> {
    /// Evaluates the status of the game based on the current state,
    /// in accordance with the rules of Bloxorz.
    pub fn status(self) -> Status<'a> {
        let Game { board, block } = self;
        if block.is_touching(Tile::Empty, board) {
            return Status::Loss;
        }
        if block.is_standing_on(Tile::Fragile, board) {
            return Status::Loss;
        }
        if block.is_standing_on(Tile::Goal, board) {
            return Status::Win;
        }
        Status::Active(ActiveGame { board, block })
    }
}

impl<'a> ActiveGame<'a> {
    /// Returns the result of making a move in the given direction in the current game state.
    pub fn make_move(self, direction: Direction) -> Game<'a> {
        Game {
            board: self.board,
            block: self.block.make_move(direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bloxorz_board;
    use crate::bloxorz_model::block::Orientation::*;
    use crate::bloxorz_model::game::*;
    use rstest::rstest;
    use Direction::{Down as D, Left as L, Right as R, Up as U};

    /// Returns the result of making multiple moves in the given directions in the given game.
    ///
    /// Panics if there are still moves to make after the game is won or lost.
    fn play<'a>(mut game: Game<'a>, directions: &[Direction]) -> Game<'a> {
        for (i, &direction) in directions.iter().enumerate() {
            let Status::Active(active_game) = game.status() else {
                panic!("move {i}: cannot make a move in a finished game")
            };
            game = active_game.make_move(direction);
        }
        game
    }

    fn dumbbell_board() -> Board {
        bloxorz_board![
            [# # # . . . # # $]
            [# # # ! ! ! # # #]
            [# # # ! ! ! # # #]
            [# # # . . . # # $]
        ]
    }

    #[rstest]
    #[case::top_goal(&[D, R, R, R, R, R, R, R, R, U], Block((8, 0), Upright))]
    #[case::bottom_goal(&[R, D, D, D, L, U, R, R, R, R, R, R, D, R, U, U, U, L, D, R, R, D],
        Block((8, 3), Upright))]
    fn test_winning_play(#[case] directions: &[Direction], #[case] final_block: Block) {
        let board = dumbbell_board();
        let result = play(
            Game {
                board: &board,
                block: Block((0, 0), Upright),
            },
            directions,
        );
        assert_eq!(result.block, final_block);
        let Status::Win = result.status() else {
            panic!("expected a win");
        };
    }

    #[rstest]
    #[case::roll_off(&[D, L], Block((-1, 1), Vertical))]
    #[case::topple_off(&[D, R, U, R], Block((2, 0), Horizontal))]
    #[case::walk_off(&[D, D, R, R], Block((3, 3), Upright))]
    #[case::unstable_from_left(&[R, D, L, D, R, U, R, R], Block((4, 1), Upright))]
    #[case::unstable_from_right(&[D, R, R, R, R, R, R, U, R, D, D, L, L, L],
        Block((3, 2), Upright))]
    fn test_losing_play(#[case] directions: &[Direction], #[case] final_block: Block) {
        let board = dumbbell_board();
        let result = play(
            Game {
                board: &board,
                block: Block((0, 0), Upright),
            },
            directions,
        );
        assert_eq!(result.block, final_block);
        let Status::Loss = result.status() else {
            panic!("expected a loss");
        };
    }
}
