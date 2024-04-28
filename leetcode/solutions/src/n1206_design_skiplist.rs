#![cfg(feature = "rand")]
/**
 * [1206] Design Skiplist
 *
 * Design a Skiplist without using any built-in libraries.
 * A skiplist is a data structure that takes O(log(n)) time to add, erase and search. Comparing with treap and red-black tree which has the same function and performance, the code length of Skiplist can be comparatively short and the idea behind Skiplists is just simple linked lists.
 * For example, we have a Skiplist containing [30,40,50,60,70,90] and we want to add 80 and 45 into it. The Skiplist works this way:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/27/1506_skiplist.gif" style="width: 500px; height: 173px;" /><br />
 * <small>Artyom Kalinin [CC BY-SA 3.0], via <a href="https://commons.wikimedia.org/wiki/File:Skip_list_add_element-en.gif" target="_blank" title="Artyom Kalinin [CC BY-SA 3.0 (https://creativecommons.org/licenses/by-sa/3.0)], via Wikimedia Commons">Wikimedia Commons</a></small>
 * You can see there are many layers in the Skiplist. Each layer is a sorted linked list. With the help of the top layers, add, erase and search can be faster than O(n). It can be proven that the average time complexity for each operation is O(log(n)) and space complexity is O(n).
 * See more about Skiplist: <a href="https://en.wikipedia.org/wiki/Skip_list" target="_blank">https://en.wikipedia.org/wiki/Skip_list</a>
 * Implement the Skiplist class:
 *
 *     Skiplist() Initializes the object of the skiplist.
 *     bool search(int target) Returns true if the integer target exists in the Skiplist or false otherwise.
 *     void add(int num) Inserts the value num into the SkipList.
 *     bool erase(int num) Removes the value num from the Skiplist and returns true. If num does not exist in the Skiplist, do nothing and return false. If there exist multiple num values, removing any one of them is fine.
 *
 * Note that duplicates may exist in the Skiplist, your code needs to handle this situation.
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["Skiplist", "add", "add", "add", "search", "add", "search", "erase", "erase", "search"]
 * [[], [1], [2], [3], [0], [4], [1], [0], [1], [1]]
 * Output
 * [null, null, null, null, false, null, true, false, true, false]
 * Explanation
 * Skiplist skiplist = new Skiplist();
 * skiplist.add(1);
 * skiplist.add(2);
 * skiplist.add(3);
 * skiplist.search(0); // return False
 * skiplist.add(4);
 * skiplist.search(1); // return True
 * skiplist.erase(0);  // return False, 0 is not in skiplist.
 * skiplist.erase(1);  // return True
 * skiplist.search(1); // return False, 1 has already been erased.
 *  
 * Constraints:
 *
 *     0 <= num, target <= 2 * 10^4
 *     At most 5 * 10^4 calls will be made to search, add, and erase.
 *
 */
// submission codes start here
use std::{cell::RefCell, rc::Rc};

const MAX_HEIGHT: usize = 32;

struct Node {
	val: i32,
	nexts: RefCell<Vec<Option<Rc<Self>>>>,
}
#[derive(Default)]
pub struct Skiplist {
	nexts: [Option<Rc<Node>>; MAX_HEIGHT],
	start: usize,
}

impl Skiplist {
	fn height() -> usize { 1 + rand::random::<u32>().trailing_ones() as usize }

	pub fn new() -> Self { Self::default() }
	fn _insert(nexts: &mut [Option<Rc<Node>>], cur: usize, new: &Rc<Node>) {
		for cur in (0..=cur).rev() {
			match nexts[cur].as_deref() {
				Some(inner) if inner.val < new.val => {
					return Self::_insert(&mut inner.nexts.borrow_mut(), cur, new)
				}
				_ if new.nexts.borrow().len() > cur => {
					new.nexts.borrow_mut()[cur] = nexts[cur].take();
					nexts[cur] = Some(new.clone());
				}
				_ => {}
			}
		}
	}
	pub fn add(&mut self, val: i32) {
		let hei = Self::height();
		let new = Rc::new(Node {
			val,
			nexts: RefCell::new(vec![None; hei]),
		});
		self.start = self.start.max(hei - 1);
		Self::_insert(&mut self.nexts, self.start, &new);
	}

	pub fn search(&self, k: i32) -> bool { Self::_find(&self.nexts, self.start, &k) }

	fn _find(nexts: &[Option<Rc<Node>>], cur: usize, k: &i32) -> bool {
		for cur in (0..=cur).rev() {
			match nexts[cur].as_deref() {
				Some(inner) if inner.val < *k => return Self::_find(&inner.nexts.borrow(), cur, k),
				Some(inner) if inner.val == *k => return true,
				_ => {}
			}
		}
		false
	}
	pub fn erase(&mut self, k: i32) -> bool { Self::_remove(&mut self.nexts, self.start, &k) }
	fn _remove(nexts: &mut [Option<Rc<Node>>], cur: usize, k: &i32) -> bool {
		let mut ret = false;
		for cur in (0..=cur).rev() {
			match nexts[cur].as_deref() {
				Some(inner) if inner.val < *k => {
					return Self::_remove(&mut inner.nexts.borrow_mut(), cur, k)
				}
				Some(inner) if inner.val == *k => {
					let next = inner.nexts.borrow_mut()[cur].take();
					nexts[cur] = next;
					ret = true;
				}
				_ => {}
			}
		}
		ret
	}
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1206() {
		let mut list = Skiplist::new();
		list.add(1);
		list.add(2);
		list.add(3);
		assert!(!list.search(0));
		list.add(4);
		assert!(list.search(1));
		list.erase(0);
		list.erase(1);
		assert!(!list.search(1));
	}
}
