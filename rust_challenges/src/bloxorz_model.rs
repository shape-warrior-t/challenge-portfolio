//! Bloxorz is a 2007 puzzle game about navigating a rectangular block into a hole.
//! Knowledge of the game is assumed throughout the challenge.
//! It can be played at https://www.coolmathgames.com/0-bloxorz.
//!
//! Problem: model a simplified version of Bloxorz.
//!
//! The only type of special tile included in the model is the fragile orange tile.
//! Switches (and thus, bridges, as well as the ability to split the block) are not included.

// Dependencies (later modules depend on earlier ones): board -> block -> game
mod block;
mod board;
mod game;

pub use block::{Block, Direction, Orientation, DIRECTIONS};
pub use board::{Board, Coordinates, Tile};
pub use game::{ActiveGame, Game, Status};
