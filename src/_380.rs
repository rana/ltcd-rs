use rand::{thread_rng, Rng};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// 380. Insert Delete GetRandom O(1)
///
/// Implement the RandomizedSet class:
///
/// * RandomizedSet() Initializes the RandomizedSet object.
///
/// * bool insert(int val) Inserts an item val into the set
/// if not present. Returns true if the item was not present,
/// false otherwise.
///
/// * bool remove(int val) Removes an item val from the set
/// if present. Returns true if the item was present, false
/// otherwise.
///
/// * int getRandom() Returns a random element from the current
/// set of elements (it's guaranteed that at least one element
/// exists when this method is called). Each element must have
/// the same probability of being returned.
///
/// * You must implement the functions of the class such that
/// each function works in average O(1) time complexity.
///
/// Constraints:
/// * -2^31 <= val <= 2^31 - 1
/// * At most 2 * 10^5 calls will be made to insert, remove,
/// and getRandom.
/// * There will be at least one element in the data structure
/// when getRandom is called.

struct RandomizedSet {
    vec: Vec<i32>,
    map: HashMap<i32, usize>,
    rnd: usize,
}
impl RandomizedSet {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
            // Initialize max possible capacity for time efficiency.
            map: HashMap::with_capacity(200_000),
            // `rnd` is state for a pseudo-randomization function.
            // Any non-zero initial state will do.
            rnd: 1488,
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let Entry::Vacant(e) = self.map.entry(val) {
            self.vec.push(val);
            e.insert(self.vec.len() - 1);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idx) = self.map.remove(&val) {
            // Remediate index if necessary.
            let lst = self.vec.pop().unwrap();
            if idx < self.vec.len() {
                self.vec[idx] = lst;
                self.map.insert(lst, idx);
            }
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        // Hand write pseudo-randomization function for time efficiency.
        // Linear Feedback Shift Register function.
        self.rnd ^= self.rnd << 13;
        self.rnd ^= self.rnd >> 7;
        self.rnd ^= self.rnd << 17;
        self.vec[self.rnd % self.vec.len()]
    }
}

struct RandomizedSet0 {
    vec: Vec<i32>,
    map: HashMap<i32, usize>,
}
impl RandomizedSet0 {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let Entry::Vacant(e) = self.map.entry(val) {
            self.vec.push(val);
            e.insert(self.vec.len() - 1);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idx) = self.map.remove(&val) {
            // Remediate index if necessary.
            let lst = self.vec.pop().unwrap();
            if idx < self.vec.len() {
                self.vec[idx] = lst;
                self.map.insert(lst, idx);
            }
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let idx = thread_rng().gen_range(0..self.vec.len());
        self.vec[idx]
    }
}
