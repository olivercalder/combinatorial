//! Combinatorial tools, functions, and generators.

pub mod combinations;

#[cfg(test)]
mod tests {
    use super::*;
    use combinations::Combinations;

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
