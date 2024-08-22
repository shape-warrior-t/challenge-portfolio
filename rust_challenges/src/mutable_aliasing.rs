//! Problem: given a list of immutable and mutable references to variables,
//! determine which variables violate Rust's mutability XOR aliasing requirement.
//!
//! (Posted with modifications to
//! https://codegolf.stackexchange.com/questions/274829/is-there-mutable-aliasing-in-this-list-of-variable-references.)

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Whether a variable reference is immutable or mutable.
#[derive(Clone, Copy)]
pub enum Mutability {
    Immutable,
    Mutable,
}

/// A reference to a variable, simulating Rust's `&x` and `&mut x` references.
pub struct Reference<T> {
    // The exact representation of a variable is flexible, and can be of various types.
    variable: T,
    mutability: Mutability,
}

/// A classification of a given variable's set of references,
/// based on the number of immutable and mutable references.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ReferenceSetType {
    /// No references.
    Empty,
    /// One or more immutable references.
    Aliased,
    /// Exactly one mutable reference.
    Mutable,
    /// Two or more references, at least one of which is mutable.
    MutablyAliased,
}

/// Returns the set of variables that are mutably aliased
/// (have two or more references, at least one of which is mutable)
/// in the given list of references.
pub fn mutable_aliasing_violations<T: Copy + Eq + Hash>(references: &[Reference<T>]) -> HashSet<T> {
    use Mutability as Mut;
    use ReferenceSetType as RST;
    let mut var_to_type = HashMap::new();
    for reference in references {
        let ref_set_type = var_to_type.entry(reference.variable).or_insert(RST::Empty);
        *ref_set_type = match (*ref_set_type, reference.mutability) {
            (RST::Empty, Mut::Immutable) => RST::Aliased,
            (RST::Empty, Mut::Mutable) => RST::Mutable,
            (RST::Aliased, Mut::Immutable) => RST::Aliased,
            (RST::Aliased, Mut::Mutable) => RST::MutablyAliased,
            (RST::Mutable, _) => RST::MutablyAliased,
            (RST::MutablyAliased, _) => RST::MutablyAliased,
        };
    }
    var_to_type
        .iter()
        .filter_map(|(&var, &ref_set_type)| (ref_set_type == RST::MutablyAliased).then_some(var))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::mutable_aliasing::*;
    use rstest::rstest;

    /// Creates a list of references to variables represented by `&str` names.
    ///
    /// Syntax:
    /// ```text
    /// refs![<reference in the form `&x` or `&mut x`>, ...]
    /// ```
    macro_rules! refs {
        (@ref $var:ident) => {
            Reference {
                variable: stringify!($var),
                mutability: Mutability::Immutable,
            }
        };
        (@ref mut $var:ident) => {
            Reference {
                variable: stringify!($var),
                mutability: Mutability::Mutable,
            }
        };
        ($(&$($tokens:ident)*),*) => {
            [$(refs!(@ref $($tokens)*)),*]
        };
    }

    #[rstest]
    #[case(&refs![], [])]
    #[case(&refs![&a], [])]
    #[case(&refs![&mut a], [])]
    #[case(&refs![&b, &b], [])]
    #[case(&refs![&mut b, &b], ["b"])]
    #[case(&refs![&b, &mut b], ["b"])]
    #[case(&refs![&mut b, &mut b], ["b"])]
    #[case(&refs![&c, &d], [])]
    #[case(&refs![&mut c, &mut d], [])]
    #[case(&refs![&a, &mut d, &mut d], ["d"])]
    #[case(&refs![&mut a, &d, &mut d], ["d"])]
    #[case(&refs![&mut a, &mut d, &mut d, &mut a], ["a", "d"])]
    #[case(&refs![&a, &mut d, &d, &a], ["d"])]
    #[case(&refs![&c, &mut g, &c, &c], [])]
    #[case(&refs![&c, &mut g, &mut c, &c], ["c"])]
    #[case(&refs![&a, &b, &c, &d, &e], [])]
    #[case(&refs![&mut f, &e, &e, &mut d], [])]
    #[case(&refs![&f, &e, &mut e, &d], ["e"])]
    #[case(&refs![&a, &mut g, &b, &a, &b, &b, &mut f], [])]
    #[case(&refs![&mut a, &g, &b, &a, &b, &mut b, &f], ["a", "b"])]
    #[case(&refs![&a, &a, &a, &a, &a, &a, &a], [])]
    #[case(&refs![&a, &a, &a, &mut a, &a, &a, &a], ["a"])]
    #[case(&refs![&mut a, &mut a, &mut a, &mut a, &mut a, &mut a, &mut a], ["a"])]
    #[case(&refs![&mut g, &mut g, &mut g, &mut g, &mut g, &mut g, &mut g], ["g"])]
    #[case(&refs![&a, &b, &mut c, &mut d, &e, &mut f, &g], [])]
    #[case(&refs![&a, &b, &c, &mut a, &mut b, &mut c], ["a", "b", "c"])]
    #[case(&refs![&a, &mut a, &a, &mut g, &g, &mut g], ["a", "g"])]
    fn tests<const N: usize>(#[case] references: &[Reference<&str>], #[case] expected: [&str; N]) {
        assert_eq!(
            mutable_aliasing_violations(references),
            HashSet::from(expected)
        );
    }
}
