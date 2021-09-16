/**
 * [146] LRU Cache
 *
 * Design a data structure that follows the constraints of a <a href="https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU" target="_blank">Least Recently Used (LRU) cache</a>.
 * Implement the LRUCache class:
 *
 *     LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
 *     int get(int key) Return the value of the key if the key exists, otherwise return -1.
 *     void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
 *
 * The functions <code data-stringify-type="code">get and <code data-stringify-type="code">put must each run in O(1) average time complexity.
 *  
 * Example 1:
 *
 * Input
 * ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
 * [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
 * Output
 * [null, null, null, 1, null, -1, null, -1, 3, 4]
 * Explanation
 * LRUCache lRUCache = new LRUCache(2);
 * lRUCache.put(1, 1); // cache is {1=1}
 * lRUCache.put(2, 2); // cache is {1=1, 2=2}
 * lRUCache.get(1);    // return 1
 * lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
 * lRUCache.get(2);    // returns -1 (not found)
 * lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
 * lRUCache.get(1);    // return -1 (not found)
 * lRUCache.get(3);    // return 3
 * lRUCache.get(4);    // return 4
 *
 *  
 * Constraints:
 *
 *     1 <= capacity <= 3000
 *     0 <= key <= 10^4
 *     0 <= value <= 10^5
 *     At most 2 * 10^5 calls will be made to get and put.
 *
 */
// submission codes start here

struct LRUEntry {
	key: i32,
	val: i32,
	next: *mut LRUEntry,
	prev: *mut LRUEntry,
}
use std::collections::HashMap;
struct LRUCache {
	map: HashMap<i32, Box<LRUEntry>>,
	cap: i32,
	head: Box<LRUEntry>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LRUCache {
	fn new(cap: i32) -> Self {
		unsafe {
			use std::mem::MaybeUninit;
			let mut cache = LRUCache {
				cap,
				map: HashMap::with_capacity((cap + 1) as usize),
				head: Box::new(MaybeUninit::zeroed().assume_init()), //is this safe?
			};
			cache.head.next = cache.head.as_mut();
			cache.head.prev = cache.head.as_mut();
			cache
		}
	}

	fn get(&mut self, key: i32) -> i32 {
		match self.map.get_mut(&key) {
			Some(e) => unsafe {
				Self::detach(e);
				let val = e.val;
				let e = e.as_mut() as *mut LRUEntry;
				self.append(e);
				val
			},
			None => -1,
		}
	}

	fn put(&mut self, key: i32, val: i32) {
		use std::collections::hash_map::Entry;
		match self.map.entry(key) {
			Entry::Vacant(o) => {
				let e = o.insert(Box::new(LRUEntry {
					key,
					val,
					prev: std::ptr::null_mut(),
					next: std::ptr::null_mut(),
				}));
				let e = e.as_mut() as *mut LRUEntry;

				unsafe {
					self.append(e);
					if self.map.len() > self.cap as usize {
						let to_rem = (*self.head.next).key;
						let mut e = self.map.remove(&to_rem).unwrap();
						Self::detach(e.as_mut());
					}
				}
			}
			Entry::Occupied(mut e) => {
				let e = e.get_mut();
				unsafe {
					Self::detach(e);
					e.val = val;
					let e = e.as_mut() as *mut LRUEntry;
					self.append(e);
				}
			}
		}
	}

	unsafe fn detach(e: &LRUEntry) {
		(*e.prev).next = e.next;
		(*e.next).prev = e.prev;
	}

	unsafe fn append(&mut self, e: *mut LRUEntry) {
		(*e).prev = self.head.prev;
		self.head.prev = e;
		(*e).next = self.head.as_mut();
		(*(*e).prev).next = e;
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
		let mut obj = LRUCache::new(2);
		obj.put(1, 1);
		assert_eq!(obj.get(1), 1);
		obj.put(2, 2);
		assert_eq!(obj.get(1), 1);
		obj.put(3, 3);
		assert_eq!(obj.get(2), -1);
		obj.put(4, 4);
		assert_eq!(obj.get(1), -1);
		assert_eq!(obj.get(3), 3);
		assert_eq!(obj.get(4), 4);
	}
}
