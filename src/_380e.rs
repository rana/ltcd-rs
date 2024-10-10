use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct RandomizedSet {
    list: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            list: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        // Check whether the key already exists.
        if self.map.contains_key(&val) {
            return false;
        }

        // Push to the list.
        self.list.push(val);

        // Insert to the map.
        self.map.insert(val, self.list.len() - 1);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        // Check whether the value exists.
        if let Some(&idx) = self.map.get(&val) {
            // Swap the removed value with the list last element.
            let last_element = *self.list.last().unwrap();
            self.list[idx] = last_element;
            self.map.insert(last_element, idx);

            // Remove the list last element.
            self.list.pop();
            // Remove the map last element.
            self.map.remove(&last_element);

            return true;
        }

        false
    }

    fn get_random(&self) -> i32 {
        // Use rnd and list to get random element.
        let mut rng = thread_rng();
        let idx_rnd = rng.gen_range(0..self.list.len());
        self.list[idx_rnd]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut random_set = RandomizedSet::new();
        assert_eq!(random_set.insert(1), true); // Valid insert
        assert_eq!(random_set.insert(1), false); // Invalid insert (duplicate)
    }

    #[test]
    fn test_remove() {
        let mut random_set = RandomizedSet::new();
        random_set.insert(1);
        assert_eq!(random_set.remove(1), true); // Valid removal
        assert_eq!(random_set.remove(1), false); // Invalid removal (already removed)
    }

    #[test]
    fn test_get_random_single_element() {
        let mut random_set = RandomizedSet::new();
        random_set.insert(1);
        assert_eq!(random_set.get_random(), 1); // With only 1 element, it must return 1
    }

    #[test]
    fn test_get_random_multiple_elements() {
        let mut random_set = RandomizedSet::new();
        random_set.insert(1);
        random_set.insert(2);
        let random_value = random_set.get_random();
        assert!(random_value == 1 || random_value == 2); // Random should be 1 or 2
    }

    #[test]
    fn test_insert_and_remove_sequence() {
        let mut random_set = RandomizedSet::new();
        assert_eq!(random_set.insert(10), true);
        assert_eq!(random_set.insert(20), true);
        assert_eq!(random_set.insert(30), true);

        assert_eq!(random_set.remove(20), true); // Remove 20
        assert_eq!(random_set.insert(40), true); // Insert 40

        let random_value = random_set.get_random();
        assert!(random_value == 10 || random_value == 30 || random_value == 40);
        // 20 is no longer present
    }

    #[test]
    fn test_remove_invalid_element() {
        let mut random_set = RandomizedSet::new();
        random_set.insert(1);
        assert_eq!(random_set.remove(2), false); // Invalid removal, 2 doesn't exist
    }
}
