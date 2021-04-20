/**
 * [173] Binary Search Tree Iterator
 *
 * Implement the BSTIterator class that represents an iterator over the <a href="https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR)">in-order traversal</a> of a binary search tree (BST):
 *
 *     BSTIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
 *     boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
 *     int next() Moves the pointer to the right, then returns the number at the pointer.
 *
 * Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.
 * You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/25/bst-tree.png" style="width: 189px; height: 178px;" />
 * Input
 * ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
 * [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
 * Output
 * [null, 3, 7, true, 9, true, 15, true, 20, false]
 * Explanation
 * BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
 * bSTIterator.next();    // return 3
 * bSTIterator.next();    // return 7
 * bSTIterator.hasNext(); // return True
 * bSTIterator.next();    // return 9
 * bSTIterator.hasNext(); // return True
 * bSTIterator.next();    // return 15
 * bSTIterator.hasNext(); // return True
 * bSTIterator.next();    // return 20
 * bSTIterator.hasNext(); // return False
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 10^5].
 *     0 <= Node.val <= 10^6
 *     At most 10^5 calls will be made to hasNext, and next.
 *
 *  
 * Follow up:
 *
 *     Could you implement next() and hasNext() to run in average O(1) time and use O(h) memory, where h is the height of the tree?
 *
 */
use super::util::tree::TreeNode;

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
struct BSTIterator {
	st: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
impl BSTIterator {
	fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
		let mut st = Vec::new();
		while let Some(no) = root {
			root = no.borrow_mut().left.take();
			st.push(no);
		}
		BSTIterator { st }
	}

	fn next(&mut self) -> i32 {
		let no = self.st.pop().unwrap();
		let mut r = no.borrow_mut().right.take();
		while let Some(o) = r {
			r = o.borrow_mut().left.take();
			self.st.push(o);
		}

		let val = no.borrow_mut().val;
		val
	}

	fn has_next(&self) -> bool { !self.st.is_empty() }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_173() {
		let mut it = BSTIterator::new(tree![2, 1, 3]);
		assert!(it.has_next());

		assert_eq!(it.next(), 1);
		assert_eq!(it.next(), 2);
		assert_eq!(it.next(), 3);
		assert!(!it.has_next());
	}
}
