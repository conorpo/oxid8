use hashlink::LinkedHashMap;
use std::borrow::ToOwned;
use std::hash::Hash;

pub struct TwoQcache<K, V> {
    new: LinkedHashMap<K, V>,
    cold: LinkedHashMap<K, ()>,
    hot: LinkedHashMap<K, V>,
    capacity: usize,
}

impl<K: Eq + Hash + ToOwned<Owned = K>, V> TwoQcache<K, V> {
    pub fn new() -> Self {
        Self {
            new: LinkedHashMap::new(),
            cold: LinkedHashMap::new(),
            hot: LinkedHashMap::new(),

            // !todo  this value should be changed in production
            capacity: 20,
        }
    }

    // !todo  this may need slight refactoring later on

    pub fn access(&mut self, key: &K, val: V) -> Option<&V> {
        //if in hot then remove and place the reinsert the value. LRU
        if self.hot.contains_key(key) {
            self.hot.remove(key);

            self.check_capacity();
            self.hot.insert(key.to_owned(), val);
            return self.hot.get(key);
        }
        //if in new then we place the key in hot
        if self.new.contains_key(key) {
            self.new.remove(key);

            self.check_capacity();
            self.hot.insert(key.to_owned(), val);
            return self.hot.get(key);
        }
        //if in cold we also place key in hot
        if self.cold.contains_key(key) {
            self.cold.remove(key);

            self.check_capacity();
            self.hot.insert(key.to_owned(), val);
            return self.hot.get(key);
        }
        // if not found we insert into new
        self.check_capacity();
        self.new.insert(key.to_owned(), val);
        return self.new.get(key);
    }

    // !todo the self.capacity / 4 and 2  may need a change in the future
    pub fn check_capacity(&mut self) {
        //check hot capacity if it is equal to the set capacity then we can pop the front since we will be inserting after this.
        if self.hot.len() >= self.capacity {
            self.hot.pop_front();
        }

        // check capacity of new and insert overflown keys into cold.
        if self.new.len() >= self.capacity / 4 {
            if let Some((old_key, _)) = self.new.pop_front() {
                self.cold.insert(old_key, ());
            }
        }

        //check cold capacity if it is equal to the set capacity then we can pop the front since we will be inserting after this.
        if self.cold.len() >= self.capacity / 2 {
            self.cold.pop_front();
        }
    }
}
