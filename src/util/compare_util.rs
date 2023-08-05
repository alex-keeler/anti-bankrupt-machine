use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Map;

pub fn vecs_are_equal_unordered<T: Eq + Hash>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {

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