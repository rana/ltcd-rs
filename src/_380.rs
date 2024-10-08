use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            list: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        // Check if the value already exists.
        if self.map.contains_key(&val) {
            return false;
        }
        // Push the value to the back of the list.
        self.list.push(val);
        // Map the value to the list index.
        self.map.insert(val, self.list.len() - 1);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            // Swap the remove element with the last element.
            let last_element = *self.list.last().unwrap();
            self.list[idx] = last_element;
            self.map.insert(last_element, idx);

            // Remove the last element.
            self.list.pop();
            self.map.remove(&val);

            // The remove operation succeeded.
            true
        } else {
            // The remove operation did not succeed.
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_idx = rng.gen_range(0..self.list.len());
        self.list[random_idx]
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
