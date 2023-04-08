//! A combination generator for Rust

use std::collections::BTreeSet;

pub struct Combinations<T>
where
    T: Ord + Clone,
{
    elements: Vec<T>,
    positions: Vec<usize>,
    all_sizes: bool,
    done: bool,
}

fn vec_to_sorted_set<T: Ord + Clone>(elements: &Vec<T>) -> Vec<T> {
    elements
        .iter()
        .cloned()
        .collect::<BTreeSet<T>>()
        .iter()
        .cloned()
        .collect::<Vec<T>>()
}

impl<T: Ord + Clone> Combinations<T> {
    pub fn all(elements: &Vec<T>) -> Self {
        Combinations {
            elements: vec_to_sorted_set(elements),
            positions: Vec::new(),
            all_sizes: true,
            done: false,
        }
    }

    pub fn with_size(elements: &Vec<T>, size: usize) -> Self {
        Combinations {
            elements: vec_to_sorted_set(elements),
            positions: (0..size).collect(),
            all_sizes: false,
            done: false,
        }
    }

    fn move_to_next_set_size(&mut self) -> bool {
        if self.positions.len() >= self.elements.len() {
            return false;
        }
        self.positions
            .iter_mut()
            .enumerate()
            .for_each(|(index, pos)| *pos = index);
        self.positions.push(self.positions.len());
        true
    }

    fn move_to_next_position(&mut self) -> bool {
        let length = self.positions.len();
        for index in (0..self.positions.len()).rev() {
            let cur_position = *self.positions.get(index).unwrap();
            if self.elements.len() == 0 || cur_position >= self.elements.len() - 1 {
                continue;
            }
            if index == length - 1 || cur_position < self.positions.get(index + 1).unwrap() - 1 {
                let mut next_position = cur_position + 1;
                *self.positions.get_mut(index).unwrap() = next_position;
                for i in index + 1..length {
                    next_position += 1;
                    *self.positions.get_mut(i).unwrap() = next_position;
                }
                return true;
            }
        }
        false
    }

    fn get_current_combination(&mut self) -> Option<Vec<T>> {
        if self.done || self.positions.len() > self.elements.len() {
            return None;
        }
        Some(
            self.positions
                .iter()
                .map(|p| self.elements.get(*p).unwrap().clone())
                .collect::<Vec<T>>(),
        )
    }
}

impl<T: Ord + Clone> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let combo = self.get_current_combination();
        if self.move_to_next_position() == false {
            if self.all_sizes == false || self.move_to_next_set_size() == false {
                self.done = true;
            }
        }
        combo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_vec_to_sorted_set() {
        assert_eq!(vec![1, 2, 3, 4], vec_to_sorted_set(&vec![1, 2, 3, 4]));
        assert_eq!(vec![1, 2, 3, 4], vec_to_sorted_set(&vec![2, 3, 1, 4]));
        assert_eq!(
            vec![1, 2, 3, 4],
            vec_to_sorted_set(&vec![2, 1, 3, 1, 4, 2, 2, 3])
        );
    }

    #[test]
    fn test_combinations_all() {
        let combos = Combinations::all(&vec![2, 4, 3, 1, 2, 2, 1]);
        assert_eq!(combos.elements, vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.all_sizes, true);
        assert_eq!(combos.done, false);
    }

    #[test]
    fn test_combinations_with_size() {
        let combos = Combinations::with_size(&vec![2, 4, 3, 1, 2, 2, 1], 3);
        assert_eq!(combos.elements, vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, vec![0, 1, 2]);
        assert_eq!(combos.all_sizes, false);
        assert_eq!(combos.done, false);
    }

    #[test]
    fn test_combinations_move_to_next_set_size() {
        let mut combos = Combinations::all(&Vec::<i64>::new());
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), false);
        let mut combos = Combinations::all(&vec![1]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_set_size(), false);
        let mut combos = Combinations::all(&vec![1, 2, 3, 4]);
        assert_eq!(combos.positions, Vec::new());
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0]);
        combos.positions[0] = 4;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0, 1]);
        combos.positions[0] = 5;
        combos.positions[1] = 2;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0, 1, 2]);
        combos.positions[0] = 3;
        combos.positions[1] = 7;
        combos.positions[2] = 1;
        assert_eq!(combos.move_to_next_set_size(), true);
        assert_eq!(combos.positions, vec![0, 1, 2, 3]);
        combos.positions[0] = 0;
        combos.positions[1] = 0;
        combos.positions[2] = 0;
        combos.positions[2] = 0;
        assert_eq!(combos.move_to_next_set_size(), false);
    }

    #[test]
    fn test_combinations_move_to_next_position() {
        let mut combos = Combinations::with_size(&Vec::<i64>::new(), 1);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = Combinations::with_size(&vec![1], 1);
        assert_eq!(combos.positions, vec![0]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = Combinations::with_size(&vec![1, 2, 3, 4], 2);
        assert_eq!(combos.positions, vec![0, 1]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![2, 3]);
        assert_eq!(combos.move_to_next_position(), false);
        let mut combos = Combinations::with_size(&vec![1, 2, 3, 4], 3);
        assert_eq!(combos.positions, vec![0, 1, 2]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 1, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![0, 2, 3]);
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.positions, vec![1, 2, 3]);
        assert_eq!(combos.move_to_next_position(), false);
    }

    #[test]
    fn test_combinations_get_current_combination() {
        let mut combos = Combinations::with_size(&vec![1, 1, 2, 3, 5, 8], 3);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 3]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 2, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 3, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 3, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![1, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 3, 5]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 3, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![2, 5, 8]));
        assert_eq!(combos.move_to_next_position(), true);
        assert_eq!(combos.get_current_combination(), Some(vec![3, 5, 8]));
        assert_eq!(combos.move_to_next_position(), false);
        combos.done = true;
        assert_eq!(combos.get_current_combination(), None);
    }

    #[test]
    fn test_combinations_next() {
        let mut combos = Combinations::with_size(&vec![1, 2, 3], 0);
        assert_eq!(combos.next(), Some(Vec::new()));
        assert_eq!(combos.next(), None);
        let mut combos = Combinations::with_size(&vec![1, 2, 3], 4);
        assert_eq!(combos.next(), None);
        let mut combos: Combinations<u64> = Combinations::with_size(&Vec::new(), 0);
        assert_eq!(combos.next(), Some(Vec::new()));
        assert_eq!(combos.next(), None);
        let combos = Combinations::all(&vec![1, 1, 2, 3, 5]);
        assert_eq!(
            combos.collect::<Vec<Vec<u64>>>(),
            vec![
                Vec::new(),
                vec![1],
                vec![2],
                vec![3],
                vec![5],
                vec![1, 2],
                vec![1, 3],
                vec![1, 5],
                vec![2, 3],
                vec![2, 5],
                vec![3, 5],
                vec![1, 2, 3],
                vec![1, 2, 5],
                vec![1, 3, 5],
                vec![2, 3, 5],
                vec![1, 2, 3, 5],
            ]
        );
    }
}
