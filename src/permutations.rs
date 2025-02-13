#[derive(Debug)]
struct Entry {
    prev: Option<usize>, // TODO: use smaller u8, u16, etc. if num elements is small enough
    next: Option<usize>, // TODO: stop using Option<>, just use an extra footer similar to the
                         // current header
}

struct AvailableList {
    entries: Vec<Entry>, // TODO: use boxed array instead
}

impl AvailableList {
    /// Create a new `AvailableList` corresponding to a list of entries with the given number of
    /// elements.
    fn new(num_elements: usize) -> Self {
        let mut entries: Vec<Entry> = Vec::with_capacity(num_elements + 1); // need space for head
        entries.push(Entry {
            // the 0th entry will act as a "head", but with removal of head.next working
            // consistently as with entries at other indices.
            prev: None,
            next: Some(1),
        });
        for i in 1..num_elements {
            entries.push(Entry {
                prev: Some(i - 1),
                next: Some(i + 1),
            });
        }
        entries.push(Entry {
            prev: Some(num_elements - 1),
            next: None,
        });
        Self { entries }
    }

    /// Remove the first available entry from the list and returns its index, if one exists.
    fn remove_first(&mut self) -> Option<usize> {
        let Some(i) = self.entries[0].next else {
            return None;
        };
        self.remove(i);
        Some(i)
    }

    /// Remove the entry at the given index from the list. Entries must be re-added in the reverse
    /// order from which they were removed, else the list will be corrupted.
    fn remove(&mut self, i: usize) {
        debug_assert!(i > 0); // not the header
        debug_assert!(i < self.entries.len()); // TODO: use footer, and check < length - 1
        let prev = self.entries[i].prev;
        let next = self.entries[i].next;
        if let Some(p) = prev {
            self.entries[p].next = next;
        }
        if let Some(n) = next {
            self.entries[n].prev = prev;
        }
    }

    /// Add the entry at the given index back into the list. The entry must have been previously
    /// removed from the list, and all removed entries must be re-added in the reverse order from
    /// which they were removed, else the list will be corrupted.
    fn add(&mut self, i: usize) {
        debug_assert!(i > 0); // not the header
        debug_assert!(i < self.entries.len()); // TODO:, use footer, and check < length - 1
        if let Some(p) = self.entries[i].prev {
            self.entries[p].next = Some(i);
        }
        if let Some(n) = self.entries[i].next {
            self.entries[n].prev = Some(i);
        }
    }

    /// Adds the entry at the given index back into the list, and removes the next available entry
    /// after it, if one exists. The state of the available list must be identical to what it was
    /// when the entry at the given index was previously removed.
    fn swap_for_next(&mut self, i: usize) -> Option<usize> {
        debug_assert!(i > 0); // not the header
        debug_assert!(i < self.entries.len()); // TODO:, use footer, and check < length - 1
        let next = self.entries[i].next;
        self.add(i);
        let Some(n) = next else {
            return None;
        };
        self.remove(n);
        Some(n)
    }
}

/// An iterator which generates permutations in lexicographic order over a list of elements.
///
/// There exist efficient algorithms for generating permutations, such as Heap's Algorithm or the
/// Steinhaus-Johnson-Trotter algorithm, which require swapping only two elements to generate each
/// subsequent permutation. However, these algorithms do not produce permutations in lexicographic
/// order.
///
/// Instead, this iterator uses something which resembles a combination of a linked list and an
/// explicit free list to allow advancing to the next permutation in amortized `O(Mlog(M)/N)` time, <-- this isn't true, it's more complicated than that, and better
/// where `M` is the length of the permutation, and `N` is the size of the set over which
/// permutations are being generated, where `M <= N`. Worst case
///
/// # Examples
///
/// ```
/// use combinatorial::Permutations;
///
/// let mut xyz_perms = Permutations::new(vec!['x', 'y', 'z']);
/// assert_eq!(xyz_perms.next(), Some(vec!['x', 'y', 'z']));
/// assert_eq!(xyz_perms.next(), Some(vec!['x', 'z', 'y']));
/// assert_eq!(xyz_perms.next(), Some(vec!['y', 'x', 'z']));
/// assert_eq!(xyz_perms.next(), Some(vec!['y', 'z', 'x']));
/// assert_eq!(xyz_perms.next(), Some(vec!['z', 'x', 'y']));
/// assert_eq!(xyz_perms.next(), Some(vec!['z', 'y', 'x']));
/// assert_eq!(xyz_perms.next(), None);
/// ```
pub struct Permutations<T> {
    elements: Vec<T>, // TODO: use boxed array instead
    avail_list: AvailableList,
    stack: Vec<usize>,
    perm_length: usize,
    all_sizes: bool,
    done: bool,
}

impl<T: Clone> Permutations<T> {
    /// Creates a new `Permutations` iterator which will yield all permutations in lexicographic
    /// order of all the elements in the given iterable, relative to the original order of those
    /// elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use combinatorial::Permutations;
    ///
    /// let mut perms = Permutations::new(1..=3);
    /// assert_eq!(perms.next(), Some(vec![1, 2, 3]));
    /// assert_eq!(perms.next(), Some(vec![1, 3, 2]));
    /// assert_eq!(perms.next(), Some(vec![2, 1, 3]));
    /// assert_eq!(perms.next(), Some(vec![2, 3, 1]));
    /// assert_eq!(perms.next(), Some(vec![3, 1, 3]));
    /// assert_eq!(perms.next(), Some(vec![3, 2, 1]));
    /// assert_eq!(perms.next(), None);
    ///
    /// let mut perms = Permutations::new(vec!["Alice", "Eve", "Bob"]);
    /// assert_eq!(perms.next(), Some(vec!["Alice", "Eve", "Bob"]));
    /// assert_eq!(perms.next(), Some(vec!["Alice", "Bob", "Eve"]));
    /// assert_eq!(perms.next(), Some(vec!["Eve", "Alice", "Bob"]));
    /// assert_eq!(perms.next(), Some(vec!["Eve", "Bob", "Alice"]));
    /// assert_eq!(perms.next(), Some(vec!["Bob", "Alice", "Eve"]));
    /// assert_eq!(perms.next(), Some(vec!["Bob", "Eve", "Alice"]));
    /// assert_eq!(perms.next(), None);
    ///
    /// let mut perms = Permutations::new(1..1);
    /// assert_eq!(perms.next(), Some(Vec::new()));
    /// assert_eq!(perms.next(), None);
    /// ```
    pub fn new(elements: impl IntoIterator<Item = T>) -> Self {
        let elems = elements.into_iter().collect::<Vec<T>>();
        let length = elems.len();
        Permutations::from_vec_with_size_constraints(elems, length, false)
    }

    pub fn of_length(elements: impl IntoIterator<Item = T>, perm_length: usize) -> Self {
        let elems = elements.into_iter().collect::<Vec<T>>();
        Permutations::from_vec_with_size_constraints(elems, perm_length, false)
    }

    pub fn all(elements: impl IntoIterator<Item = T>) -> Self {
        let elems = elements.into_iter().collect::<Vec<T>>();
        Permutations::from_vec_with_size_constraints(elems, 0, true)
    }

    fn from_vec_with_size_constraints(
        elements: Vec<T>,
        perm_length: usize,
        all_sizes: bool,
    ) -> Self {
        let avail_list = AvailableList::new(elements.len());
        let perm_capacity = if all_sizes {
            elements.len()
        } else {
            perm_length
        };
        let stack: Vec<usize> = Vec::with_capacity(perm_capacity);
        let mut perms = Self {
            elements,
            avail_list,
            stack,
            perm_length,
            all_sizes,
            done: false,
        };
        perms.fill_remaining_perm();
        perms
    }

    /// Repeatedly removes the first available entry from the available list and adds them to the
    /// current permutation stack until the stack contains `self.perm_length` entries. If the
    /// permutation length is greater than the number of elements, this is impossible, so returns
    /// false, and sets `self.done = true`. Returns true if the stack has been fully populated.
    fn fill_remaining_perm(&mut self) -> bool {
        if self.perm_length > self.elements.len() {
            self.done = true;
            return false;
        }
        for _ in self.stack.len()..self.perm_length {
            let Some(i) = self.avail_list.remove_first() else {
                panic!(
                    "avail_list: {:?}\nstack: {:?}",
                    self.avail_list.entries, self.stack
                );
            };
            // unwrap must succeed since we checked that self.perm_length <= self.elements.len()
            self.stack.push(i);
        }
        true
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    /// Returns the next permutation and advances the internal iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let perm: Vec<T> = self
            .stack
            .iter()
            .map(|i| self.elements[i - 1].clone()) // use i - 1 since header consumes element 0
            .collect();
        loop {
            let Some(curr_last) = self.stack.pop() else {
                // we're out of entries in the existing permutation, and none of them had available
                // next entries, so we've exhausted every permutation of this length.
                if !self.all_sizes {
                    self.done = true;
                    return None;
                }
                self.perm_length += 1;
                // we know stack is empty, so populate an initial permutation of the new size
                if !self.fill_remaining_perm() {
                    // couldn't populate an initial permutation of this new size, so we're out of
                    // permutations.

                    debug_assert!(self.done); // check that fill_remaining_perm set done to true
                    return None;
                }
                // we're on a new permutation length, and filled the stack with the next
                // permutation, so break out of the loop and return the current permutation we
                // already set aside.
                break;
            };
            let Some(next) = self.avail_list.swap_for_next(curr_last) else {
                // there's no available next for the element of the permutation we popped off the
                // stack, so try again with the previous element in the permutation.
                continue;
            };
            if !self.fill_remaining_perm() {
                // Couldn't fill remaining permutation. XXX: can this ever happen?
                // Re-add next to the available list, and try with the next element in the stack.
                self.avail_list.add(next);
                continue;
            }
            self.stack.push(next);
            break;
        }
        Some(perm)
    }
}
