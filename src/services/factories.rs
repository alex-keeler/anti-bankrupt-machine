pub trait Factory<K, V> {
    fn get_instance(key: K) -> Option<V>;
}