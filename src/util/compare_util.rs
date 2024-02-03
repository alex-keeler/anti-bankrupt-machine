use std::collections::HashMap;
use std::hash::Hash;

pub fn _vecs_are_equal_unordered<T: Eq + Hash>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
    fn build_compare_map<T: Eq + Hash>(vec: &Vec<T>) -> HashMap<&T, usize> {
        let mut map = HashMap::new();

        for entry in vec {
            *map.entry(entry).or_insert(0) += 1;
        }

        map
    }

    if vec1.len() != vec2.len() {
        return false;
    }

    build_compare_map(vec1) == build_compare_map(vec2)
}

pub fn _vecs_are_equal_ordered<T: Eq>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {

    if vec1.len() != vec2.len() {
        return false;
    }

    for (entry1, entry2) in vec1.iter().zip(vec2.iter()) {
        if entry1 != entry2 {
            return false;
        }
    }

    return true;
}