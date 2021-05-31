use std::collections::hash_map::Entry;
/**
 * [146] LRU Cache
 *
 * Design and implement a data structure for <a href="https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU" target="_blank">Least Recently Used (LRU) cache</a>. It should support the following operations: get and put.
 *
 * get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.<br />
 * put(key, value) - Set or insert the value if the key is not already present. When the cache reached its capacity, it should invalidate the least recently used item before inserting a new item.
 *
 * The cache is initialized with a positive capacity.
 *
 * Follow up:<br />
 * Could you do both operations in O(1) time complexity?
 *
 * Example:
 *
 *
 * LRUCache cache = new LRUCache( 2 /* capacity */ );
 *
 * cache.put(1, 1);
 * cache.put(2, 2);
 * cache.get(1);       // returns 1
 * cache.put(3, 3);    // evicts key 2
 * cache.get(2);       // returns -1 (not found)
 * cache.put(4, 4);    // evicts key 1
 * cache.get(1);       // returns -1 (not found)
 * cache.get(3);       // returns 3
 * cache.get(4);       // returns 4
 *
 *
 *  
 *
 */
use std::collections::HashMap;
#[allow(dead_code)]
pub struct Solution {}

// submission codes start here

struct LRUCache {
	ks: Vec<i32>,
	m: HashMap<i32, i32>,
	ind: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LRUCache {
	fn new(capacity: i32) -> Self {
		LRUCache {
			ks: Vec::with_capacity(capacity as usize),
			m: HashMap::new(),
			ind: 0,
		}
	}

	fn get(&self, key: i32) -> i32 {
		match self.m.get(&key) {
			Some(&i) => i,
			None => -1,
		}
	}

	fn put(&mut self, key: i32, value: i32) {
		// TODO: 重入值的时候刷新不了
		let mut ent = self.m.entry(key);
		match ent {
			Entry::Vacant(v) => {
				v.insert(value);
			}
			Entry::Occupied(o) => {
				// 这里应该要刷新生命周期的, 如果是vec的话就不好做
				// o.insert(value) => 这里insert也不行...
				*o.into_mut() = value;
			}
		};
	}
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_146() {
		let mut obj = LRUCache::new(3);
		obj.put(3, 20);
		assert_eq!(obj.get(3), 20);
		obj.put(4, 40);
		obj.put(8, 30);
	}
}
