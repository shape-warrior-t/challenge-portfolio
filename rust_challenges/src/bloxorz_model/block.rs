//! Module for the player-controlled block.

use crate::bloxorz_model::board::{Board, Coordinates, Tile};

/// A direction in which the block can be moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

/// The orientation of the block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    /// Standing up, covering a 1×1 area.
    Upright,
    /// Lying down, covering a 2×1 area.
    Horizontal,
    /// Lying down, covering a 1×2 area.
    Vertical,
}

/// The rectangular block that the player controls.
///
/// The coordinates of the block refer to the top left square of its covered area.
/// The block is not, by itself, associated with a board --
/// on its own, it can move to any pair of integer coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Block(pub Coordinates, pub Orientation);

impl Block {
    /// Returns the result of moving the block once in the given direction,
    /// following the block movement mechanics of Bloxorz.
    pub fn make_move(self, direction: Direction) -> Block {
        // Movement:
        // Upright: | Horizontal: | Vertical:
        //          |             |
        //   U      |             |
        //   u      |  Uu         |  U
        // LlSRr    | LSsR        | LSR
        //   D      |  Dd         | lsr
        //   d      |             |  D
        //
        // S: start; L: left; R: right; U: up; D: down
        // Capital letters indicate the square referred to by the block's coordinates.
        use Direction::*;
        use Orientation::*;
        let Block((x, y), curr_orientation) = self;
        let (dx, dy, new_orientation) = match (curr_orientation, direction) {
            (Upright, Left) => (-2, 0, Horizontal),
            (Upright, Right) => (1, 0, Horizontal),
            (Upright, Up) => (0, -2, Vertical),
            (Upright, Down) => (0, 1, Vertical),
            (Horizontal, Left) => (-1, 0, Upright),
            (Horizontal, Right) => (2, 0, Upright),
            (Horizontal, Up) => (0, -1, Horizontal),
            (Horizontal, Down) => (0, 1, Horizontal),
            (Vertical, Left) => (-1, 0, Vertical),
            (Vertical, Right) => (1, 0, Vertical),
            (Vertical, Up) => (0, -1, Upright),
            (Vertical, Down) => (0, 2, Upright),
        };
        Block((x + dx, y + dy), new_orientation)
    }

    /// Returns the coordinates of both squares covered by the block.
    ///
    /// For upright blocks, returns the same pair of coordinates twice.
    fn full_coordinates(self) -> [Coordinates; 2] {
        let Block((x, y), orientation) = self;
        let (dx, dy) = match orientation {
            Orientation::Upright => (0, 0),
            Orientation::Horizontal => (1, 0),
            Orientation::Vertical => (0, 1),
        };
        [(x, y), (x + dx, y + dy)]
    }

    /// Returns whether any part of the block would be touching a tile of the given type
    /// if it were on the given board.
    pub fn is_touching(self, tile: Tile, board: &Board) -> bool {
        self.full_coordinates()
            .iter()
            .any(|&coordinates| board.tile_at(coordinates) == tile)
    }

    /// Returns whether the block would be standing upright on a tile of the given type
    /// if it were on the given board.
    pub fn is_standing_on(self, tile: Tile, board: &Board) -> bool {
        let Block(_, orientation) = self;
        orientation == Orientation::Upright && self.is_touching(tile, board)
    }
}

#[cfg(test)]
mod tests {
    use crate::bloxorz_board;
    use crate::bloxorz_model::block::*;
    use rstest::rstest;
    use Direction::*;
    use Orientation::*;
    use Tile::*;

    #[rstest]
    #[case::upright_left    (Block((0, 0), Upright),    Left,  Block((-2, 0), Horizontal))]
    #[case::upright_right   (Block((3, 1), Upright),    Right, Block((4, 1),  Horizontal))]
    #[case::upright_up      (Block((0, 0), Upright),    Up,    Block((0, -2), Vertical))]
    #[case::upright_down    (Block((4, 1), Upright),    Down,  Block((4, 2),  Vertical))]
    #[case::horizontal_left (Block((0, 3), Horizontal), Left,  Block((-1, 3), Upright))]
    #[case::horizontal_right(Block((5, 9), Horizontal), Right, Block((7, 9),  Upright))]
    #[case::horizontal_up   (Block((6, 1), Horizontal), Up,    Block((6, 0),  Horizontal))]
    #[case::horizontal_down (Block((2, 6), Horizontal), Down,  Block((2, 7),  Horizontal))]
    #[case::vertical_left   (Block((1, 6), Vertical),   Left,  Block((0, 6),  Vertical))]
    #[case::vertical_right  (Block((5, 3), Vertical),   Right, Block((6, 3),  Vertical))]
    #[case::vertical_up     (Block((3, 0), Vertical),   Up,    Block((3, -1), Upright))]
    #[case::vertical_down   (Block((5, 8), Vertical),   Down,  Block((5, 10), Upright))]
    fn test_make_move(#[case] block: Block, #[case] direction: Direction, #[case] expected: Block) {
        assert_eq!(block.make_move(direction), expected);
    }

    fn slanted_rectangle_board() -> Board {
        bloxorz_board![
            [. # . .]
            [# # # .]
            [. # # #]
            [. . # .]
        ]
    }

    #[rstest]
    #[case::upright_not_touching       (Block((1, 2),  Upright),    Empty, false)]
    #[case::upright_touching           (Block((3, 1),  Upright),    Empty, true)]
    #[case::horizontal_not_touching    (Block((0, 1),  Horizontal), Empty, false)]
    #[case::horizontal_left_touching   (Block((1, 3),  Horizontal), Empty, true)]
    #[case::horizontal_right_touching  (Block((3, 2),  Horizontal), Empty, true)]
    #[case::horizontal_all_touching    (Block((-1, 2), Horizontal), Empty, true)]
    #[case::vertical_not_touching      (Block((2, 1),  Vertical),   Empty, false)]
    #[case::vertical_top_touching      (Block((1, -1), Vertical),   Empty, true)]
    #[case::vertical_bottom_touching   (Block((3, 2),  Vertical),   Empty, true)]
    #[case::vertical_all_touching      (Block((3, 0),  Vertical),   Empty, true)]
    fn test_is_touching(#[case] block: Block, #[case] tile: Tile, #[case] expected: bool) {
        assert_eq!(
            block.is_touching(tile, &slanted_rectangle_board()),
            expected
        );
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
    #[case::upright_not_touching       (Block((0, 0),  Upright),    Goal,    false)]
    #[case::upright_touching_fragile   (Block((4, 1),  Upright),    Fragile, true)]
    #[case::upright_touching_goal      (Block((8, 3),  Upright),    Goal,    true)]
    #[case::upright_touching_incorrect (Block((8, 3),  Upright),    Fragile, false)]
    #[case::horizontal_not_touching    (Block((6, 3),  Horizontal), Goal,    false)]
    #[case::horizontal_left_touching   (Block((5, 1),  Horizontal), Fragile, false)]
    #[case::horizontal_right_touching  (Block((7, 0),  Horizontal), Goal,    false)]
    #[case::horizontal_all_touching    (Block((3, 2),  Horizontal), Fragile, false)]
    #[case::vertical_not_touching      (Block((0, 1),  Vertical),   Fragile, false)]
    #[case::vertical_top_touching      (Block((8, 0),  Vertical),   Goal,    false)]
    #[case::vertical_bottom_touching   (Block((8, 2),  Vertical),   Goal,    false)]
    #[case::vertical_all_touching      (Block((3, 1),  Vertical),   Fragile, false)]
    fn test_is_standing_on(#[case] block: Block, #[case] tile: Tile, #[case] expected: bool) {
        assert_eq!(block.is_standing_on(tile, &dumbbell_board()), expected);
    }
}
