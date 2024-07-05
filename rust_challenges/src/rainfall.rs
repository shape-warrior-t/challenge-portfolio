//! Problem (modified from
//! https://codereview.meta.stackexchange.com/questions/6872/august-2016-community-challenge):
//!
//! Consider a rectangular region of land consisting of square cells at different altitudes,
//! represented by a grid of numbers.
//!
//! For example:
//! ```text
//! 3 1 4 2 5 9
//! 2 6 5 3 5 8
//! 9 7 9 3 1 3
//! ```
//!
//! Rain that falls on a given cell will flow to other cells based on the relative altitudes of
//! the cell and its orthogonal (not diagonal) neighbors:
//! - Requirement: for a given cell and its neighbors, there is a unique cell of lowest altitude.
//! Regions that violate this requirement are considered invalid.
//! - If a cell has a higher altitude than one or more of its neighbors,
//! then rain will flow from the cell to the neighbor with the lowest altitude.
//! - If a cell has a lower altitude than all of its neighbors,
//! then it is a sink, and rain will collect in the cell.
//!
//! For any given cell, there is a unique sink that the cell _drains into_ --
//! rain that falls on the cell will eventually flow to and collect in the sink.
//!
//! For example, in the region from above:
//! ```text
//! 3 1 4 2 5 9
//! 2 6 5 3 5 8
//! 9 7 9 3 1 3
//! ```
//! rain that falls on
//!
//! `cell (2, 1) (altitude 5)` will flow to
//!
//! `cell (3, 1) (altitude 3)`, then flow to and collect in
//!
//! `cell (3, 0) (altitude 2)`,
//!
//! so `cell (2, 1)` drains into `cell (3, 0)`. Note that every sink drains into itself.
//!
//! Every sink is associated with a _basin_: the set of cells that drain into the sink.
//!
//! Task: identify the basins in a given region.
//!
//! For our example region:
//! ```text
//! 3 1 4 2 5 9
//! 2 6 5 3 5 8
//! 9 7 9 3 1 3
//! ```
//! we have the following sinks:
//! ```text
//! . a . b . .
//! c . . . . .
//! . . . . d .
//! ```
//! and the following associated basins:
//! ```text
//! a a a b b b
//! c a b b d d
//! c a d d d d
//! ```

use crate::grid::Grid;
use itertools::Itertools;
use std::fmt::{self, Debug};

type CellCoordinates = (i32, i32);

/// A basin, identified by the sink that the basin is associated with.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Basin {
    /// The coordinates of the sink that all cells in the basin drain into.
    sink: CellCoordinates,
}

impl Debug for Basin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.sink)
    }
}

type Altitude = i32;

type Region = Grid<Altitude>;

/// Identifies the basins in the given region.
///
/// On success, returns a grid that maps the coordinates of a cell
/// to the basin that the cell belongs to.
///
/// Fails for invalid regions, returning the coordinates of the cell
/// where the unique lowest altitude requirement is found to be violated.
pub fn identify_basins(region: &Region) -> Result<Grid<Basin>, CellCoordinates> {
    let mut basins = Grid::filled(None, region.dimensions());
    for (cell, _) in region.enumerate() {
        identify_basin_at(region, cell, &mut basins)?;
    }
    Ok(basins.map(|basin| basin.unwrap()))
    /*
        Time complexity analysis:
        Let `c` be the number of cells in the region.
        This function completes in `O(c)` time in the worst case --
        for an `n×n` square region, this translates to a time complexity of `O(n^2)`.
        - Disregarding work done in `identify_basin_at`, `identify_basins` completes in
        `O(c)` time -- creating `basins`, executing the for loop, and mapping over `basins`
        can all be done in `O(c)` time.
        - `identify_basin_at` relies on memoization to achieve an efficient time complexity.
        Memoized calls complete in `O(1)` time,
        so the cost can be absorbed into the cost at the call site.
        There are at most `c` non-memoized calls -- one for each cell.
        Non-memoized calls also complete in `O(1)`, disregarding work done in recursive calls:
            - All non-`O(1)` functions in `locally_lowest_cell` operate on a maximum of 5 items,
            so the function as a whole is `O(1)`.
            - Everything else completes in `O(1)` time.
    */
}

/// Identifies the basin for the cell at the given coordinates in the given region,
/// recording the basin in `basins` if not already recorded.
///
/// Fails if the region is discovered to be invalid, returning the coordinates of the cell
/// where the unique lowest altitude requirement is found to be violated.
fn identify_basin_at(
    region: &Region,
    cell: CellCoordinates,
    basins: &mut Grid<Option<Basin>>,
) -> Result<(), CellCoordinates> {
    if basins[cell].is_none() {
        let lowest = locally_lowest_cell(region, cell)?;
        let cell_is_sink = cell == lowest;
        if cell_is_sink {
            basins[cell] = Some(Basin { sink: cell });
        } else {
            identify_basin_at(region, lowest, basins)?;
            basins[cell] = basins[lowest];
        }
    }
    Ok(())
}

/// Returns the coordinates of the cell of lowest altitude
/// between the cell at the given coordinates and its neighbors.
///
/// Fails if there is more than one cell of lowest altitude,
/// returning the input coordinates to indicate
/// a violation of the unique lowest altitude requirement (and thus, an invalid region).
fn locally_lowest_cell(
    region: &Region,
    cell: CellCoordinates,
) -> Result<CellCoordinates, CellCoordinates> {
    let neighborhood = neighborhood_coordinates(cell)
        .into_iter()
        .filter_map(|coordinates| {
            let &altitude = region.get(coordinates)?;
            Some((coordinates, altitude))
        });
    unique_lowest_altitude_cell(neighborhood).ok_or(cell)
}

/// Given the coordinates of a cell, returns the possible coordinates of the cell and its neighbors.
/// Will return out-of-bounds coordinates for cells on the edge of the region.
fn neighborhood_coordinates(cell: CellCoordinates) -> [CellCoordinates; 5] {
    let (x, y) = cell;
    [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

/// Returns the coordinates of the cell of lowest altitude
/// based on the given `(coordinate, altitude)` pairs,
/// or None if there are multiple cells of lowest altitude.
fn unique_lowest_altitude_cell(
    coordinate_altitude_pairs: impl Iterator<Item = (CellCoordinates, Altitude)>,
) -> Option<CellCoordinates> {
    coordinate_altitude_pairs
        .min_set_by_key(|&(_coordinates, altitude)| altitude)
        .into_iter()
        .exactly_one()
        .ok()
        .map(|(coordinates, _altitude)| coordinates)
}

#[cfg(test)]
mod tests {
    use crate::rainfall::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(3, 0)]
    #[case(0, 3)]
    fn test_empty_regions(#[case] width: usize, #[case] height: usize) {
        let dimensions = (width, height);
        let actual = identify_basins(&Grid::filled(0, dimensions));
        let placeholder_basin = Basin { sink: (0, 0) };
        let expected = Ok(Grid::filled(placeholder_basin, dimensions));
        assert_eq!(actual, expected);
    }

    /// Test case macro for Rainfall.
    ///
    /// Syntax for success cases:
    /// ```text
    /// test! {<test name>: [
    ///     [<altitude>, ...],
    ///     ...
    /// ] => ok [
    ///     [<basin name>, ...],
    ///     ...
    /// ], sinks {
    ///     <basin name>: <coordinates of sink associated with basin>,
    ///     ...
    /// }}
    /// ```
    ///
    /// Syntax for failure cases:
    /// ```text
    /// test! {<test name>: [
    ///     [<altitude>, ...],
    ///     ...
    /// ] => err}
    /// ```
    /// For failure cases, the return value is checked to ensure that a violation of
    /// the unique lowest altitude requirement actually occurs at the indicated location.
    /// There might be multiple such locations, but only one will ever be returned,
    /// so we can't just hardcode the expected return value.
    macro_rules! test {
        ($name:ident: $region:expr => ok $basins:expr, sinks {$($var:ident: $sink:expr,)*}) => {
            #[test]
            fn $name() {
                $(let $var = Basin { sink: $sink };)*
                let region = Grid::from_2d_array($region);
                let actual = identify_basins(&region);
                let expected = Ok(Grid::from_2d_array($basins));
                assert_eq!(actual, expected);
            }
        };
        ($name:ident: $region:expr => err) => {
            #[test]
            fn $name() {
                let region = Grid::from_2d_array($region);
                let cell = identify_basins(&region).unwrap_err();
                let lowest_altitudes = neighborhood_coordinates(cell)
                    .into_iter()
                    .filter_map(|neighbor_cell| region.get(neighbor_cell))
                    .min_set();
                assert!(lowest_altitudes.len() > 1, "no violation at {cell:?}");
            }
        };
    }

    test! {problem_description_example: [
        [3, 1, 4, 2, 5, 9],
        [2, 6, 5, 3, 5, 8],
        [9, 7, 9, 3, 1, 3],
    ] => ok [
        [a, a, a, b, b, b],
        [c, a, b, b, d, d],
        [c, a, d, d, d, d],
    ], sinks {
        a: (1, 0),
        b: (3, 0),
        c: (0, 1),
        d: (4, 2),
    }}

    test! {corner_sinks: [
        [0, 1, 1, 0],
        [2, 3, 2, 3],
        [1, 2, 3, 2],
        [0, 3, 1, 0],
    ] => ok [
        [a, a, b, b],
        [a, a, b, b],
        [c, c, d, d],
        [c, c, d, d],
    ], sinks {
        a: (0, 0),
        b: (3, 0),
        c: (0, 3),
        d: (3, 3),
    }}

    test! {spiral: [
        [-12, -11, -10,  -9,  -8],
        [  5,   4,   3,   2,  -7],
        [  6,  -1,   0,   1,  -6],
        [  7,  -2,  -3,  -4,  -5],
        [  8,   9,  10,  11,  12],
    ] => ok [
        [a, a, a, a, a],
        [a, a, a, a, a],
        [a, a, a, a, a],
        [a, a, a, a, a],
        [a, a, a, a, a],
    ], sinks {
        a: (0, 0),
    }}

    test! {strips: [
        [-1, -2, -3, -4, -5, -7],
        [-1, -2, -3, -4, -5, -6],
        [-1, -2, -3, -4, -5, -8],
    ] => ok [
        [a, a, a, a, a, a],
        [b, b, b, b, b, b],
        [b, b, b, b, b, b],
    ], sinks {
        a: (5, 0),
        b: (5, 2),
    }}

    test! {single: [
        [0],
    ] => ok [
        [a],
    ], sinks {
        a: (0, 0),
    }}

    test! {single_row: [
        [1, 0, 2],
    ] => ok [
        [a, a, a],
    ], sinks {
        a: (1, 0),
    }}

    test! {single_column: [
        [0],
        [2],
        [1],
    ] => ok [
        [a],
        [a],
        [b],
    ], sinks {
        a: (0, 0),
        b: (0, 2),
    }}

    // Violation at cell (2, 0) (altitude 4).
    test! {problem_description_example_invalid_modification: [
        [3, 1, 4, 1, 5, 9],
        [2, 6, 5, 3, 5, 8],
        [9, 7, 9, 3, 2, 3],
    ] => err}

    // Violation at the top cells.
    test! {unclear_sink: [
        [0, 0],
        [1, 1],
    ] => err}

    // Violation at the center cells.
    test! {ambiguous_corner_sinks: [
        [-1,  0,  0, -1],
        [ 0,  1,  1,  0],
        [ 0,  1,  1,  0],
        [-1,  0,  0, -1],
    ] => err}

    // Violation at cell (5, 1) (altitude -6).
    test! {ambiguous_strips: [
        [-1, -2, -3, -4, -5, -7],
        [-1, -2, -3, -4, -5, -6],
        [-1, -2, -3, -4, -5, -7],
    ] => err}

    // Violation at every cell.
    test! {all_equal: [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ] => err}
}
