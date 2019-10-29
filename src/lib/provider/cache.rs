use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;

pub struct Cache<K, V>
{
    cache: HashMap<K, Option<V>>,
}
impl<K,V> Cache<K,V> {
    pub fn new() -> Self
    where
        K: Eq + Hash
    {
        Self {cache: HashMap::<K, Option<V>>::new() }
    }

    pub fn get<L>(&mut self, key: &K, loader: &L) -> Option<&V>
    where
        L: Fn(&K)->Option<V>,
        K: Eq + Hash + Copy
    {
        self.cache.entry(*key).or_insert(loader(key)).as_ref()
    }

    #[allow(dead_code)]
    pub fn get_mem_usage(&self) -> usize {
        std::mem::size_of_val(&self.cache)
    }
}
