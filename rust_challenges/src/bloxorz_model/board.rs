//! Module for specifying a Bloxorz stage.

use crate::grid::Grid;

/// A square of terrain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    /// Empty space.
    Empty,
    /// A standard gray tile.
    Regular,
    /// A fragile orange tile.
    Fragile,
    /// A hole that the block needs to fall through to win the stage.
    Goal,
}

pub type Coordinates = (i32, i32);

/// The terrain of a Bloxorz stage.
///
/// Note that boards are allowed to have multiple goals, unlike in the actual game.
pub struct Board(pub Grid<Tile>);

impl Board {
    /// The tile at the given coordinates.
    ///
    /// Out-of-bounds locations are treated as containing empty space.
    pub fn tile_at(&self, coordinates: Coordinates) -> Tile {
        let Board(grid) = self;
        grid.get(coordinates).copied().unwrap_or(Tile::Empty)
    }
}

/// Creates a board for a Bloxorz stage.
///
/// Syntax:
/// ```text
/// island_grid![
///     [<tile> ...]
///     ...
/// ]
/// ```
/// with the following symbols for tiles: \
/// `.` Empty \
/// `#` Regular \
/// `!` Fragile \
/// `$` Goal
#[macro_export]
macro_rules! bloxorz_board {
    (@tile .) => {Tile::Empty};
    (@tile #) => {Tile::Regular};
    (@tile !) => {Tile::Fragile};
    (@tile $) => {Tile::Goal};
    ($([$($tile:tt)*])*) => {
        {
            use $crate::bloxorz_model::{Tile, Board};
            Board($crate::grid::Grid::from_2d_array([$([$(bloxorz_board!(@tile $tile)),*]),*]))
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::bloxorz_model::board::*;
    use rstest::rstest;

    fn dumbbell_board() -> Board {
        bloxorz_board![
            [# # # . . . # # $]
            [# # # ! ! ! # # #]
            [# # # ! ! ! # # #]
            [# # # . . . # # $]
        ]
    }

    #[rstest]
    #[case::out_of_bounds_left((-5, 2), Tile::Empty)]
    #[case::out_of_bounds_down((4, 4),  Tile::Empty)]
    #[case::regular           ((0, 2),  Tile::Regular)]
    #[case::empty             ((3, 3),  Tile::Empty)]
    #[case::fragile           ((5, 1),  Tile::Fragile)]
    #[case::top_goal          ((8, 0),  Tile::Goal)]
    #[case::bottom_goal       ((8, 3),  Tile::Goal)]
    fn test_tile_at(#[case] coordinates: Coordinates, #[case] expected: Tile) {
        assert_eq!(dumbbell_board().tile_at(coordinates), expected);
    }
}
