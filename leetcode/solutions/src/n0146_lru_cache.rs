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
type ListEntry = (i32, i32, usize, usize);
struct LRUCache1 {
	map: std::collections::HashMap<i32, usize>,
	cap: usize,
	list: Vec<ListEntry>, // (k,v, prev,next)
}
impl LRUCache1 {
	fn new(cap: i32) -> Self {
		Self {
			map: Default::default(),
			cap: cap as usize + 1,
			list: vec![(0, 0, 0, 0)],
		}
	}

	fn get(&mut self, key: i32) -> i32 {
		match self.map.get(&key) {
			Some(&e) => {
				self.update(e);
				self.list[e].1
			}
			None => -1,
		}
	}

	fn put(&mut self, key: i32, val: i32) {
		use std::collections::hash_map::Entry;
		match self.map.entry(key) {
			Entry::Vacant(ent) => {
				let e = if self.list.len() == self.cap {
					let avail = self.list[0].3;
					Self::detach(&mut self.list, avail);
					ent.insert(avail);
					self.map.remove(&self.list[avail].0.clone());
					self.list[avail] = (key, val, 0, 0);
					avail
				} else {
					ent.insert(self.list.len());
					self.list.push((key, val, 0, 0));
					self.list.len() - 1
				};
				self.append(e);
			}
			Entry::Occupied(e) => {
				let e = *e.get();
				self.list[e].1 = val;
				self.update(e);
			}
		}
	}
	fn detach(list: &mut [ListEntry], e: usize) {
		let (_, _, prv, next) = list[e];
		list[prv].3 = next;
		list[next].2 = prv;
	}
	fn update(&mut self, e: usize) {
		let list = &mut self.list;
		Self::detach(list, e);
		self.append(e);
	}
	fn append(&mut self, e: usize) {
		let list = &mut self.list;
		list[e].3 = 0;
		list[e].2 = list[0].2;
		let prv = list[e].2;
		list[prv].3 = e;
		list[0].2 = e;
	}
}

struct LRUCache {
	map: std::collections::HashMap<i32, (i32, i32)>, // (val, cnt)
	cap: i32,
	visited: std::collections::VecDeque<i32>,
}

impl LRUCache {
	fn new(cap: i32) -> Self {
		Self {
			cap,
			map: std::collections::HashMap::with_capacity(cap as usize),
			visited: Default::default(),
		}
	}

	fn get(&mut self, key: i32) -> i32 {
		match self.map.get_mut(&key) {
			Some((val, cnt)) => {
				*cnt += 1;
				self.visited.push_back(key);
				*val
			}
			_ => -1,
		}
	}

	fn put(&mut self, key: i32, val: i32) {
		use std::collections::hash_map::Entry;
		match self.map.entry(key) {
			Entry::Vacant(ent) => {
				ent.insert((val, 1));
				while self.map.len() as i32 > self.cap {
					let del = self.visited.pop_front().unwrap();
					let cnt = self.map.get_mut(&del).unwrap();
					cnt.1 -= 1;
					if cnt.1 == 0 {
						self.map.remove(&del);
					}
				}
			}
			Entry::Occupied(mut e) => {
				let (v, cnt) = e.get_mut();
				*v = val;
				*cnt += 1;
			}
		}
		self.visited.push_back(key);
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

		let mut obj = LRUCache::new(2);
		obj.put(2, 1);
		obj.put(3, 2);
		assert_eq!(obj.get(3), 2);
		assert_eq!(obj.get(2), 1);
		obj.put(4, 3);
		assert_eq!(obj.get(2), 1);
		assert_eq!(obj.get(3), -1);
		assert_eq!(obj.get(4), 3);
	}
}
