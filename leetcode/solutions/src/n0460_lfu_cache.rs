/**
 * [460] LFU Cache
 *
 * Design and implement a data structure for a <a href="https://en.wikipedia.org/wiki/Least_frequently_used" target="_blank">Least Frequently Used (LFU)</a> cache.
 * Implement the LFUCache class:
 *
 *     LFUCache(int capacity) Initializes the object with the capacity of the data structure.
 *     int get(int key) Gets the value of the key if the key exists in the cache. Otherwise, returns -1.
 *     void put(int key, int value) Update the value of the key if present, or inserts the key if not already present. When the cache reaches its capacity, it should invalidate and remove the least frequently used key before inserting a new item. For this problem, when there is a tie (i.e., two or more keys with the same frequency), the least recently used key would be invalidated.
 *
 * To determine the least frequently used key, a use counter is maintained for each key in the cache. The key with the smallest use counter is the least frequently used key.
 * When a key is first inserted into the cache, its use counter is set to 1 (due to the put operation). The use counter for a key in the cache is incremented either a get or put operation is called on it.
 * The functions <code data-stringify-type="code">get and <code data-stringify-type="code">put must each run in O(1) average time complexity.
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
 * [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
 * Output
 * [null, null, null, 1, null, -1, 3, null, -1, 3, 4]
 * Explanation
 * // cnt(x) = the use counter for key x
 * // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
 * LFUCache lfu = new LFUCache(2);
 * lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
 * lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
 * lfu.get(1);      // return 1
 *                  // cache=[1,2], cnt(2)=1, cnt(1)=2
 * lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
 *                  // cache=[3,1], cnt(3)=1, cnt(1)=2
 * lfu.get(2);      // return -1 (not found)
 * lfu.get(3);      // return 3
 *                  // cache=[3,1], cnt(3)=2, cnt(1)=2
 * lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
 *                  // cache=[4,3], cnt(4)=1, cnt(3)=2
 * lfu.get(1);      // return -1 (not found)
 * lfu.get(3);      // return 3
 *                  // cache=[3,4], cnt(4)=1, cnt(3)=3
 * lfu.get(4);      // return 4
 *                  // cache=[4,3], cnt(4)=2, cnt(3)=3
 *
 *  
 * Constraints:
 *
 *     0 <= capacity <= 10^4
 *     0 <= key <= 10^5
 *     0 <= value <= 10^9
 *     At most 2 * 10^5 calls will be made to get and put.
 *
 *  
 * <span style="display: none;"> </span>
 */
pub struct Solution {}

// submission codes start here

use std::collections::{BTreeSet, HashMap};
struct LFUCache {
	cap: i32,
	tick: i32,
	map: HashMap<i32, (i32, i32, i32)>, // freq, tick, key
	queue: BTreeSet<(i32, i32, i32)>,   // freq, tick, val
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
	fn new(cap: i32) -> Self {
		Self {
			cap,
			tick: 0,
			map: HashMap::new(),
			queue: BTreeSet::new(),
		}
	}

	fn get(&mut self, key: i32) -> i32 {
		self.tick += 1;
		match self.map.get_mut(&key) {
			Some(e) => {
				self.queue.remove(&(e.0, e.1, key));
				self.queue.insert((e.0 + 1, self.tick, key));
				*e = (e.0 + 1, self.tick, e.2);
				e.2
			}
			_ => -1,
		}
	}

	fn put(&mut self, key: i32, val: i32) {
		self.tick += 1;
		if let Some(e) = self.map.get_mut(&key) {
			self.queue.remove(&(e.0, e.1, key));
			self.queue.insert((e.0 + 1, self.tick, key));
			*e = (e.0 + 1, self.tick, val);
		} else if self.cap > 0 {
			if self.cap == self.map.len() as i32 {
				// pop_first?
				let e = self.queue.iter().next().copied().unwrap();
				self.queue.remove(&e);
				self.map.remove(&e.2);
			}
			self.queue.insert((1, self.tick, key));
			self.map.insert(key, (1, self.tick, val));
		}
	}
}
