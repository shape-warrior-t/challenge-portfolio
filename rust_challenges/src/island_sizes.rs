//! Problem (based on
//! https://codegolf.stackexchange.com/questions/6979/code-golf-count-islands):
//! Consider a rectangular grid, where each square is either water or land.
//! The squares of land can be partitioned into _islands_:
//! groups of land squares connected orthogonally or diagonally.
//! Given such a rectangular grid, find the sizes of each of its islands.
//!
//! In the following grid (`.` denotes water and `#` denotes land):
//! ```text
//! # # # . # # #
//! # # . . . # #
//! # . . # . . #
//! . . # . # . .
//! # . . # . . #
//! # # . . . # #
//! # # # . # # #
//! ```
//! we have the following islands:
//! ```text
//! a a a . b b b
//! a a . . . b b
//! a . . c . . b
//! . . c . c . .
//! d . . c . . e
//! d d . . . e e
//! d d d . e e e
//! ```
//! with sizes 6, 6, 4, 6, and 6.

use crate::grid::Grid;
use std::collections::VecDeque;

/// The possible square types.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Water,
    Land,
}

/// Creates a grid of water and land squares.
///
/// Syntax:
/// ```text
/// island_grid![
///     [<`.` for water, `#` for land> ...]
///     ...
/// ]
/// ```
///
/// To create the grid in the problem description:
/// ```text
/// island_grid![
///     [# # # . # # #]
///     [# # . . . # #]
///     [# . . # . . #]
///     [. . # . # . .]
///     [# . . # . . #]
///     [# # . . . # #]
///     [# # # . # # #]
/// ]
/// ```
#[macro_export]
macro_rules! island_grid {
    (@square .) => {Square::Water};
    (@square #) => {Square::Land};
    ($([$($square:tt)*])*) => {
        {
            use $crate::island_sizes::Square;
            $crate::grid::Grid::from_2d_array([$([$(island_grid!(@square $square)),*]),*])
        }
    };
}

/// Returns the sizes of the islands in the given grid (in no particular order).
pub fn island_sizes(grid: &Grid<Square>) -> Vec<usize> {
    let mut visited = Grid::filled(false, grid.dimensions());
    grid.enumerate()
        .filter_map(|(index, _)| visit_island(grid, index, &mut visited))
        .collect()
    /*
        Time complexity analysis:
        Let `s` be the number of squares in the grid.
        This function completes in `O(s)` time in the worst case --
        for an `n×n` square grid, this translates to a time complexity of `O(n^2)`.
        - Note that all operations of `VisitTracker` complete in `O(1)` time.
        - Creating `visited`, iterating over `grid`, and collecting into a `Vec`
          can all be done in `O(s)` time, disregarding work done in `visit_island`.
        - The work done across all `visit_island` calls takes `O(s)` time:
            - The code outside the while loop takes `O(1)` time,
              and executes at most `s` times, taking `O(s)` time across all calls.
            - The while loop condition takes `O(1)` time,
              so the cost can be absorbed into the loop body / post-loop return statement.
            - The loop body takes `O(s)` time across all calls:
                - The loop body takes `O(1)` time to complete.
                  Note that `NEIGHBOR_DISPLACEMENTS` has a fixed 8 elements.
                - The loop body executes at most `s` times across all calls,
                  since a square can only be visited (and thus, added into a tracker's queue) once.
    */
}

type SquareIndex = (i32, i32);

#[rustfmt::skip]
const NEIGHBOR_DISPLACEMENTS: [SquareIndex; 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

/// Visits every square in the island containing the square at the given index,
/// and returns the number of squares visited.
/// Returns None if the square at the given index is a water square or has already been visited.
fn visit_island(
    grid: &Grid<Square>,
    index: SquareIndex,
    visited: &mut Grid<bool>,
) -> Option<usize> {
    let mut tracker = VisitTracker::new(grid, visited);
    if tracker.visit(index).is_err() {
        return None;
    }
    while let Some((x, y)) = tracker.queue.pop_front() {
        let neighbor_indices = NEIGHBOR_DISPLACEMENTS
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy));
        for neighbor_index in neighbor_indices {
            _ = tracker.visit(neighbor_index);
        }
    }
    Some(tracker.num_visited)
}

/// Data type for keeping track of visited squares.
struct VisitTracker<'a> {
    /// The grid whose squares are being visited.
    grid: &'a Grid<Square>,
    /// A grid indicating which squares have been visited by any tracker.
    visited: &'a mut Grid<bool>,
    /// The number of squares visited by this tracker.
    num_visited: usize,
    /// A queue of indices of visited squares whose neighbors still need visiting.
    queue: VecDeque<SquareIndex>,
}

impl<'a> VisitTracker<'a> {
    /// Creates a new tracker that has not yet visited any squares.
    fn new(grid: &'a Grid<Square>, visited: &'a mut Grid<bool>) -> VisitTracker<'a> {
        VisitTracker {
            grid,
            visited,
            num_visited: 0,
            queue: VecDeque::new(),
        }
    }

    /// Visits the square at the given index, updating the tracker's info appropriately.
    ///
    /// Fails without updating the tracker if the index is out of bounds,
    /// the square is a water square, or the square has already been visited.
    fn visit(&mut self, index: SquareIndex) -> Result<(), ()> {
        let Some(&square) = self.grid.get(index) else {
            return Err(());
        };
        if square == Square::Land && !self.visited[index] {
            self.visited[index] = true;
            self.num_visited += 1;
            self.queue.push_back(index);
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::island_sizes::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(3, 0)]
    #[case(0, 3)]
    fn test_empty_regions(#[case] width: usize, #[case] height: usize) {
        let dimensions = (width, height);
        let actual = island_sizes(&Grid::filled(Square::Land, dimensions));
        assert_eq!(actual, []);
    }

    #[rstest]
    #[case::problem_description_example(island_grid![
        [# # # . # # #]
        [# # . . . # #]
        [# . . # . . #]
        [. . # . # . .]
        [# . . # . . #]
        [# # . . . # #]
        [# # # . # # #]
    ], [4, 6, 6, 6, 6])]
    #[case::chevrons(island_grid![
        [. # # .]
        [# . . #]
        [. . . .]
        [. # # .]
        [# . . #]
    ], [4, 4])]
    #[case::isolated(island_grid![
        [. . . . . . .]
        [. # . . . # .]
        [. . . # . . .]
        [. # . . . # .]
        [. . . . . . .]
    ], [1, 1, 1, 1, 1])]
    #[case::checkerboard(island_grid![
        [. # . #]
        [# . # .]
        [. # . #]
        [# . # .]
    ], [8])]
    #[case::spiral(island_grid![
        [# . . # # . .]
        [# . # . . # .]
        [# . # . . # .]
        [# . . # . # .]
        [. # . . . # .]
        [. . # # # . .]
        [. . . . . . .]
    ], [17])]
    #[case::question_mark(island_grid![
        [. . . . .]
        [. # # . .]
        [. . . # .]
        [. . # . .]
        [. # . . .]
        [. . . . .]
        [. # . . .]
        [. . . . .]
    ], [1, 5])]
    #[case::lisp(island_grid![
        [. # . . . . . . # # # . . . # # . . . . # . # . # .]
        [# . . # . . . . . . # . . . . # . . . . # . # . . #]
        [# . # # # . . . # # # . . . . # . . . . # # # . . #]
        [# . . # . . . . . . # . . . . # . . . . . . # . . #]
        [. # . . . . . . # # # . . . # # # . . . . . # . # .]
    ], [5, 5, 5, 8, 9, 11])]
    #[case::single(island_grid![
        [#]
    ], [1])]
    #[case::single_row(island_grid![
        [# . # . # .]
    ], [1, 1, 1])]
    #[case::single_column(island_grid![
        [.]
        [.]
        [#]
        [#]
        [#]
    ], [3])]
    #[case::all_water(island_grid![
        [. . . . . .]
        [. . . . . .]
    ], [])]
    #[case::all_land(island_grid![
        [# # # # #]
        [# # # # #]
        [# # # # #]
    ], [15])]
    fn standard_tests<const N: usize>(#[case] grid: Grid<Square>, #[case] expected: [usize; N]) {
        let mut actual = island_sizes(&grid);
        actual.sort();
        assert_eq!(actual, expected);
    }
}
