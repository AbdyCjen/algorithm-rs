/**
 * [222] Count Complete Tree Nodes
 *
 * Given the root of a complete binary tree, return the number of the nodes in the tree.
 * According to <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">Wikipedia</a>, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 * Design an algorithm that runs in less than <code data-stringify-type="code">O(n) time complexity.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/complete.jpg" style="width: 372px; height: 302px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: 6
 *
 * <strong class="example">Example 2:
 *
 * Input: root = []
 * Output: 0
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 5 * 10^4].
 *     0 <= Node.val <= 5 * 10^4
 *     The tree is guaranteed to be complete.
 *
 */
pub struct Solution {}
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
use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::count2(Self::height(&root), &root)
	}

	fn count2(h: i32, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
		match root {
			Some(root) => {
				let root = root.borrow();
				if Self::height(&root.right) == h - 1 {
					(1 << (h - 1)) + Self::count2(h - 1, &root.right)
				} else {
					(1 << (h - 2)) + Self::count2(h - 1, &root.left)
				}
			}
			None => 0,
		}
	}

	fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
		match root {
			Some(root) => Self::height(&root.borrow().left) + 1,
			None => 0,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_222() {
		assert_eq!(Solution::count_nodes(tree![1, 2, 3, 4, 5, 6]), 6);
		assert_eq!(Solution::count_nodes(tree![]), 0);
		assert_eq!(Solution::count_nodes(tree![1]), 1);
	}
}
