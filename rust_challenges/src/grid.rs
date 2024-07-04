//! Helper module that provides a 2D list type.

use itertools::Itertools;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

/// A 2D list.
///
/// Indices are `(x, y)` tuples, with `(0, 0)` signifying the top-left element.
///
/// Data is stored in row-major order,
/// and all iteration over the grid is in row-major order.
#[derive(PartialEq, Eq)]
pub struct Grid<T> {
    /// The elements of the grid, stored contiguously in a 1D `Vec`.
    data: Vec<T>,
    /// The horizontal size of the grid.
    width: usize,
    /// The vertical size of the grid.
    height: usize,
}

impl<T: Clone> Grid<T> {
    /// Constructs a grid filled with the given value and dimensions.
    pub fn filled(value: T, dimensions: (usize, usize)) -> Grid<T> {
        let (width, height) = dimensions;
        Grid {
            data: vec![value; width * height],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    /// Constructs a grid representing the same 2D list as the given 2D array.
    pub fn from_2d_array<const W: usize, const H: usize>(arr: [[T; W]; H]) -> Grid<T> {
        Grid {
            data: arr.into_iter().flatten().collect(),
            width: W,
            height: H,
        }
    }

    /// The horizontal size of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The vertical size of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// A `(width, height)` tuple describing the size of the grid.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Converts a 2D index into a 1D index for the grid's data `Vec`, if possible.
    fn convert_index_to_1d<I: GridIndex>(&self, index: I) -> Result<usize, I::Err> {
        index.to_1d_index(self.width, self.height)
    }

    /// Returns a reference to the element with the given index,
    /// or None if the index is out of bounds.
    pub fn get(&self, index: impl GridIndex) -> Option<&T> {
        let index = self.convert_index_to_1d(index).ok()?;
        Some(&self.data[index])
    }

    /// Returns a mutable reference to the element with the given index,
    /// or None if the index is out of bounds.
    pub fn get_mut(&mut self, index: impl GridIndex) -> Option<&mut T> {
        let index = self.convert_index_to_1d(index).ok()?;
        Some(&mut self.data[index])
    }

    /// Returns an `(index, element)` iterator over the grid.
    pub fn enumerate<I: GridIndex>(&self) -> impl Iterator<Item = (I, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(index, element)| (I::from_1d_index(index, self.width, self.height), element))
    }

    /// Transforms the grid by applying `f` to each element.
    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Grid<U> {
        Grid {
            data: self.data.into_iter().map(f).collect(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T, I: GridIndex> Index<I> for Grid<T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        let index = self.convert_index_to_1d(index).unwrap();
        &self.data[index]
    }
}

impl<T, I: GridIndex> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = self.convert_index_to_1d(index).unwrap();
        &mut self.data[index]
    }
}

// Debug formatting: a grid is formatted like a 2D array.
// Each grid row is meant to take up exactly one line.
// Grids with a width or height of 0 are special-cased to make their dimensions clear.
impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.width == 0 || self.height == 0 {
            return write!(f, "<empty grid: ({}, {})>", self.width, self.height);
        }
        write!(
            f,
            "[\n{}\n]",
            (0..self.height)
                .map(|y| format!(
                    "    [{}],",
                    (0..self.width)
                        .map(|x| format!("{:?}", self[(x, y)]))
                        .join(", ")
                ))
                .join("\n")
        )
    }
}

use index::GridIndex;

/// Module defining a sealed trait for grid indices.
mod index {
    use std::fmt::{Debug, Display};

    /// Trait for types that can be used as an index for a grid.
    pub trait GridIndex {
        /// Error type returned for unsuccessful index conversions.
        type Err: Debug;

        /// Converts a 2D index into a 1D index for the data `Vec` of a grid
        /// based on the given dimensions, if possible.
        ///
        /// If successful, the returned 1D index must be in bounds.
        fn to_1d_index(self, width: usize, height: usize) -> Result<usize, Self::Err>;

        /// Converts a 1D index for the data `Vec` of a grid
        /// into a 2D index based on the given dimensions.
        ///
        /// Will only be called with 1D indices that are in bounds.
        fn from_1d_index(index: usize, width: usize, height: usize) -> Self;
    }

    impl<T: Copy + Display + TryFrom<usize, Error = E> + TryInto<usize>, E: Debug> GridIndex
        for (T, T)
    {
        type Err = IndexError<T>;

        fn to_1d_index(self, width: usize, height: usize) -> Result<usize, Self::Err> {
            let (x, y) = self;
            let index_error = || IndexError {
                x,
                y,
                width,
                height,
            };
            // If the coordinates cannot fit in a `usize`,
            // then they are definitely out of bounds (either negative or too large).
            let x: usize = x.try_into().map_err(|_| index_error())?;
            let y: usize = y.try_into().map_err(|_| index_error())?;
            if (0..width).contains(&x) && (0..height).contains(&y) {
                Ok(y * width + x)
            } else {
                Err(index_error())
            }
        }

        fn from_1d_index(index: usize, width: usize, height: usize) -> Self {
            debug_assert!(
                index < width * height,
                "index {index} out of bounds for dimensions {width} * {height}"
            );
            let (x, y) = (index % width, index / width);
            (x.try_into().unwrap(), y.try_into().unwrap())
        }
    }

    /// An error for out-of-bounds indices.
    pub struct IndexError<T> {
        /// x-coordinate of the out-of-bounds index.
        x: T,
        /// y-coordinate of the out-of-bounds index.
        y: T,
        /// Width of the grid that's being indexed.
        width: usize,
        /// Height of the grid that's being indexed.
        height: usize,
    }

    impl<T: Display> Debug for IndexError<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "index ({}, {}) out of bounds for dimensions ({}, {})",
                self.x, self.y, self.width, self.height
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    /// Example grid for tests.
    #[rustfmt::skip]
    fn grid() -> Grid<i32> {
        Grid::from_2d_array([
            [3, 1, 4],
            [1, 5, 9],
        ])
    }

    #[test]
    fn test_equality() {
        assert_eq!(grid(), grid());
    }

    #[test]
    fn test_inequality_different_shape() {
        #[rustfmt::skip]
        let grid_transposed = Grid::from_2d_array([
            [3, 1],
            [4, 1],
            [5, 9],
        ]);
        assert_ne!(grid(), grid_transposed);
    }

    #[test]
    fn test_inequality_different_values() {
        #[rustfmt::skip]
        let grid_doubled = Grid::from_2d_array([
            [ 6,  2,  8],
            [ 2, 10, 18],
        ]);
        assert_ne!(grid(), grid_doubled);
    }

    #[test]
    fn test_filled() {
        #[rustfmt::skip]
        let expected = Grid::from_2d_array([
            [1, 1, 1],
            [1, 1, 1]
        ]);
        assert_eq!(Grid::filled(1, (3, 2)), expected);
    }

    #[test]
    fn test_dimensions() {
        let grid = grid();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.dimensions(), (3, 2));
    }

    #[rstest]
    #[case((0, 0), Some(3))]
    #[case((1, 0), Some(1))]
    #[case((2, 0), Some(4))]
    #[case((0, 1), Some(1))]
    #[case((1, 1), Some(5))]
    #[case((2, 1), Some(9))]
    #[case((3, 0), None)]
    #[case((0, 2), None)]
    #[case((6, 3), None)]
    #[case((-1, 0), None)]
    #[case((0, -1), None)]
    #[case((-3, -3), None)]
    fn test_get(#[case] index: (i32, i32), #[case] expected: Option<i32>) {
        assert_eq!(grid().get(index), expected.as_ref());
    }

    #[rstest]
    #[case((0, 0), 3)]
    #[case((2, 0), 4)]
    #[case((1, 1), 5)]
    #[should_panic(expected = "index (3, 4) out of bounds for dimensions (3, 2)")]
    #[case((3, 4), 0)]
    #[should_panic(expected = "index (-1, -2) out of bounds for dimensions (3, 2)")]
    #[case((-1, -2), 0)]
    fn test_index(#[case] index: (i32, i32), #[case] expected: i32) {
        assert_eq!(grid()[index], expected);
    }

    #[test]
    fn test_set() {
        let mut grid = grid();
        #[rustfmt::skip]
        let expected = Grid::from_2d_array([
            [3, 1, 4],
            [2, 5, 6],
        ]);
        *grid.get_mut((0, 1)).unwrap() = 2;
        grid[(2, 1)] = 6;
        assert_eq!(grid, expected);
        assert_eq!(grid.get_mut((4, 1)), None);
    }

    #[test]
    #[should_panic(expected = "index (5, -6) out of bounds for dimensions (3, 2)")]
    fn test_set_out_of_bounds() {
        grid()[(5, -6)] = 0;
    }

    #[test]
    fn test_iteration() {
        let grid = grid();
        let actual: Vec<((i32, i32), &i32)> = grid.enumerate().collect();
        let expected = vec![
            ((0, 0), &3),
            ((1, 0), &1),
            ((2, 0), &4),
            ((0, 1), &1),
            ((1, 1), &5),
            ((2, 1), &9),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_map() {
        #[rustfmt::skip]
        let grid_doubled = Grid::from_2d_array([
            [ 6,  2,  8],
            [ 2, 10, 18],
        ]);
        assert_eq!(grid().map(|n| n * 2), grid_doubled);
    }

    #[test]
    fn test_debug_formatting() {
        let actual = format!("{:?}\n", grid());
        let expected = indoc! {"
            [
                [3, 1, 4],
                [1, 5, 9],
            ]
        "};
        assert_str_eq!(actual, expected);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(3, 0)]
    #[case(0, 3)]
    fn test_debug_formatting_empty(#[case] width: usize, #[case] height: usize) {
        let actual = format!("{:?}", Grid::filled(0, (width, height)));
        let expected = format!("<empty grid: ({width}, {height})>");
        assert_str_eq!(actual, expected);
    }
}
