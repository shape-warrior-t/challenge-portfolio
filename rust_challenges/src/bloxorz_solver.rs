//! Follow-up challenge to Bloxorz Model.
//!
//! Problem: find the shortest solution to a Bloxorz level.
use crate::bloxorz_model::{Block, Direction, Game, Status, DIRECTIONS};
use std::collections::{
    hash_map::{Entry, HashMap},
    VecDeque,
};

/// Returns the shortest list of moves needed to win the given game,
/// or None if the game is unwinnable.
///
/// If there are multiple shortest solutions, one of them will be returned;
/// it's left unspecified which specific solution is returned.
pub fn solve(game: Game) -> Option<Vec<Direction>> {
    let mut queue = VecDeque::from([game]);
    // Map from a block representing a state
    // to a (move from previous state to current state, block for previous state) tuple
    // (or None is there is no previous state)
    // so that the solution can be reconstructed once a win is reached.
    let mut visited = HashMap::from([(game.block, None)]);
    while let Some(curr) = queue.pop_front() {
        match curr.status() {
            Status::Win => return Some(trace_moves(visited, curr.block)),
            Status::Loss => {}
            Status::Active(active_curr) => {
                for &direction in &DIRECTIONS {
                    let next = active_curr.make_move(direction);
                    if let Entry::Vacant(entry_for_next) = visited.entry(next.block) {
                        queue.push_back(next);
                        entry_for_next.insert(Some((direction, curr.block)));
                    }
                }
            }
        }
    }
    None
}

/// Reconstructs the moves needed to get to the state associated with the given block,
/// based on the map of given states.
fn trace_moves(
    visited: HashMap<Block, Option<(Direction, Block)>>,
    final_block: Block,
) -> Vec<Direction> {
    let mut result = VecDeque::new();
    let mut curr = final_block;
    while let Some((direction, prev)) = visited[&curr] {
        result.push_front(direction);
        curr = prev;
    }
    result.into()
}

#[cfg(test)]
mod tests {
    use crate::bloxorz_board;
    use crate::bloxorz_model::{Board, Orientation::*};
    use crate::bloxorz_solver::*;
    use rstest::rstest;

    /// Returns the result of making multiple moves in the given directions in the given game.
    ///
    /// Panics if there are still moves to make after the game is won or lost.
    fn play<'a>(mut game: Game<'a>, directions: &[Direction]) -> Game<'a> {
        for (i, &direction) in directions.iter().enumerate() {
            let Status::Active(active_game) = game.status() else {
                panic!("cannot make a move in a finished game: move {i} of {directions:?}")
            };
            game = active_game.make_move(direction);
        }
        game
    }

    #[rstest]
    #[case::instant_loss(bloxorz_board![[!]], Block((0, 0), Upright), None)]
    #[case::separated(bloxorz_board![
        [# # # . # # #]
        [# # # . # $ #]
        [# # # . # # #]
    ], Block((1, 1), Vertical), None)]
    #[case::no_goal(bloxorz_board![
        [# # # # # #]
        [# # # # # #]
        [# # # # # #]
    ], Block((2, 1), Horizontal), None)]
    #[case::slanted_rectangle(bloxorz_board![
        [. # . .]
        [# # # .]
        [. # # #]
        [. . $ .]
    ], Block((0, 1), Upright), None)]
    #[case::instant_win(bloxorz_board![[$]], Block((0, 0), Upright), Some(0))]
    #[case::dumbbell(bloxorz_board![
        [# # # . . . # # $]
        [# # # ! ! ! # # #]
        [# # # ! ! ! # # #]
        [# # # . . . # # $]
    ], Block((0, 0), Upright), Some(10))]
    #[case::plain_square(bloxorz_board![
        [# # # #]
        [# # # #]
        [# # # #]
        [# # # $]
    ], Block((0, 0), Upright), Some(4))]
    #[case::winding(bloxorz_board![
        [! ! ! # # # #]
        [! . . . . . #]
        [! . . . . . #]
        [$ # # . # # #]
        [# # # . # # .]
        [# # # . # # .]
        [# # # # # # .]
    ], Block((3, 0), Upright), Some(13))]
    #[case::circuit(bloxorz_board![
        [! ! ! ! ! ! ! !]
        [! ! ! ! ! ! ! !]
        [. . # . . # ! !]
        [! ! $ . . . ! !]
        [! ! . . . . ! !]
        [! ! # . . # ! !]
        [! ! ! ! ! ! ! !]
        [! ! ! ! ! ! ! !]
    ], Block((2, 2), Upright), Some(19))]
    #[case::switch(bloxorz_board![
        [. . . . # # # # # #]
        [! ! ! ! ! ! ! . # #]
        [! ! ! ! ! ! ! . # #]
        [! ! ! # ! ! ! $ # #]
        [! ! ! ! ! ! ! ! # #]
        [! ! ! ! ! ! ! ! # #]
    ], Block((0, 1), Vertical), Some(10))]
    #[case::many_paths(bloxorz_board![
        [# # # $ . . .]
        [# ! ! # . . .]
        [! . . ! . . .]
        [! . . ! . . .]
        [$ ! ! # # # $]
    ], Block((1, 1), Horizontal), Some(2))]
    #[case::tight_maneuvering(bloxorz_board![
        [# # # #]
        [. ! ! $]
        [. # # #]
    ], Block((0, 0), Horizontal), Some(7))]
    fn tests(
        #[case] board: Board,
        #[case] initial_block: Block,
        #[case] optimal_solution_length: Option<usize>,
    ) {
        let game = Game {
            board: &board,
            block: initial_block,
        };
        match optimal_solution_length {
            Some(length) => {
                let solution = solve(game).unwrap();
                assert_eq!(solution.len(), length, "incorrect length: {solution:?}");
                let Status::Win = play(game, &solution).status() else {
                    panic!("expected a win: {solution:?}");
                };
            }
            None => {
                if let Some(solution) = solve(game) {
                    panic!("expected no solution, got solution {solution:?}");
                }
            }
        }
    }
}
