//! Combinatorial tools, functions, and generators.

pub mod combinations;

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
/// ```
pub fn triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use combinations::Combinations;

    #[test]
    fn test_triangle_number() {
        assert_eq!(triangle_number(0), 0);
        assert_eq!(triangle_number(1), 1);
        assert_eq!(triangle_number(2), 3);
        let nums: Vec<usize> = (3..8).map(triangle_number).collect();
        assert_eq!(nums, vec![6, 10, 15, 21, 28]);
        assert_eq!(triangle_number(1000), (0..=1000).sum());
    }

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
