//! Combinatorial tools, functions, and generators.

mod combinations;
pub use combinations::Combinations;

/// Returns the `n`th triangle number.
///
/// # Examples
///
/// ```
/// use combinatorial::triangle_number;
///
/// assert_eq!(triangle_number(0), 0);
/// assert_eq!(triangle_number(1), 1);
/// assert_eq!(triangle_number(2), 3);
///
/// let nums: Vec<usize> = (3..8).map(triangle_number).collect();
/// assert_eq!(nums, vec![6, 10, 15, 21, 28]);
///
/// assert_eq!(triangle_number(1000), (0..=1000).sum());
///
/// for n in 1..=1000 {
///     assert_eq!(triangle_number(n), n + triangle_number(n - 1));
/// }
/// ```
pub fn triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}

/// Returns `n` factorial.
///
/// # Examples
///
/// ```
/// use combinatorial::factorial;
///
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(2), 2);
/// assert_eq!(factorial(3), 6);
///
/// assert_eq!(factorial(10), (1..=10).product());
///
/// for n in 1..10 {
///     assert_eq!(factorial(n), n * factorial(n - 1));
/// }
/// ```
pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// Returns the powerset of the elements in the given iterable.
///
/// # Examples
///
/// ```
/// use combinatorial::powerset;
///
/// let mut subsets = powerset(vec!['x', 'y']);
/// assert_eq!(subsets.next(), Some(Vec::new()));
/// assert_eq!(subsets.next(), Some(vec!['x']));
/// assert_eq!(subsets.next(), Some(vec!['y']));
/// assert_eq!(subsets.next(), Some(vec!['x', 'y']));
/// assert_eq!(subsets.next(), None);
///
/// assert_eq!(powerset(0..15).count(), 1 << 15);
/// ```
pub fn powerset<T: Ord + Clone>(elements: impl IntoIterator<Item = T>) -> Combinations<T> {
    Combinations::all(elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_next() {
        let mut combos = Combinations::of_size(vec![1, 2, 3], 0);
        assert_eq!(combos.next(), Some(Vec::new()));
        assert_eq!(combos.next(), None);
        let mut combos = Combinations::of_size(vec![1, 2, 3], 4);
        assert_eq!(combos.next(), None);
        let mut combos: Combinations<u64> = Combinations::of_size(Vec::new(), 0);
        assert_eq!(combos.next(), Some(Vec::new()));
        assert_eq!(combos.next(), None);
        let combos = Combinations::all("hello".chars());
        assert_eq!(
            combos.map(String::from_iter).collect::<Vec<String>>(),
            vec![
                "",
                "e",
                "h",
                "l",
                "o",
                "eh",
                "el",
                "eo",
                "hl",
                "ho",
                "lo",
                "ehl",
                "eho",
                "elo",
                "hlo",
                "ehlo",
            ]
        );
    }
}
