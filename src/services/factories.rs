pub trait Factory<K, V> {
    fn get_instance(key: K) -> Option<V>;
}

pub trait FactoryHandler<K> {
    fn supports(&self, key: &K) -> bool;
}