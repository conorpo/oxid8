use hashlink::linked_hash_map::RawEntryMut;
use hashlink::{DefaultHashBuilder, LinkedHashMap};
use std::borrow::ToOwned;
use std::hash::Hash;

pub struct TwoQcache<K, V> {
    new: LinkedHashMap<K, V>,
    cold: LinkedHashMap<K, ()>,
    hot: LinkedHashMap<K, V>,
    capacity: usize,
    //get_val:
}


impl<K: Eq + Hash + Copy, V> TwoQcache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            new: LinkedHashMap::new(),
            cold: LinkedHashMap::new(),
            hot: LinkedHashMap::new(),
            capacity
        }
    }

    pub fn access(&mut self, key: K, get_val: impl FnOnce(K) -> Result<V,()>) -> Option<&V> {
        //if in hot then remove and place the reinsert the value. LRU
        match self.hot.raw_entry_mut().from_key(&key) {
            RawEntryMut::Occupied(mut entry) => {
                entry.to_front();
                drop(entry);
                return Some(self.hot.front().unwrap().1);
            },
            RawEntryMut::Vacant(_) => {} 
        }

        if self.new.contains_key(&key) {
            return Some(self.new.get(&key).unwrap());
        }
        
        //if in cold we also place key in hot
        if self.cold.contains_key(&key) {
            self.cold.remove(&key);

            if self.hot.len() >= self.capacity {
                self.hot.pop_front();
            }

            return get_val(key).ok().map(|val| {
                self.hot.insert(key, val);
                self.hot.back().unwrap().1
            })
        }

        if self.new.len() >= self.capacity / 4 {
            if let Some((old_key, _)) = self.new.pop_front() {
                if self.cold.len() == self.capacity / 2 {
                    self.cold.pop_front();
                }
                self.cold.insert(old_key, ());
            }
        }

        return get_val(key).ok().map(|val| {
            self.new.insert(key, val);
            self.new.back().unwrap().1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    pub fn test_basics(){
        let capacity = 20;
        let mut cache: TwoQcache<u32, u32> = TwoQcache::new(capacity);
        
        fn get_true_val(key: u32) -> Result<u32,()> {
            return Ok(1);
        }

        for i in 0..(capacity as u32 / 4) {
            cache.access(i, get_true_val).unwrap();
            assert!(cache.new.contains_key(&i));
            assert!(cache.cold.is_empty());
        }
        //Hot should be full
        cache.access(99, get_true_val);
        //Now key 0 is in cold
        assert!(cache.cold.contains_key(&0));
        cache.access(0, get_true_val);
        //Now key 0 should be in hot
        assert!(cache.hot.contains_key(&0));
        assert!(!cache.cold.contains_key(&0));

    }


}






