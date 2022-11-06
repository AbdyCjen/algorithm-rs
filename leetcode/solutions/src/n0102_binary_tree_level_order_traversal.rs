/**
 * [102] Binary Tree Level Order Traversal
 *
 * Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[3],[9,20],[15,7]]
 *
 * Example 2:
 *
 * Input: root = [1]
 * Output: [[1]]
 *
 * Example 3:
 *
 * Input: root = []
 * Output: []
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 2000].
 *     -1000 <= Node.val <= 1000
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
#[allow(dead_code)]
impl Solution {
	pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
		if root.is_none() {
			return Vec::new();
		}
		use std::collections::VecDeque;
		let mut dq = VecDeque::new();
		let mut cur_lev = Vec::new();
		let mut ans = Vec::new();
		dq.push_back(root.unwrap());

		while !dq.is_empty() {
			for cur in dq.split_off(0) {
				let mut cur = cur.borrow_mut();
				cur_lev.push(cur.val);
				if let Some(l) = cur.left.take() {
					dq.push_back(l);
				}
				if let Some(r) = cur.right.take() {
					dq.push_back(r);
				}
			}
			ans.push(cur_lev.split_off(0));
		}
		ans
	}
}

// submission codes endr

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_102() {
		assert_eq!(
			Solution::level_order(tree![3, 9, 20, null, null, 15, 7]),
			matrix![[3], [9, 20], [15, 7]]
		);
		assert_eq!(Solution::level_order(tree![1]), matrix![[1]]);
		assert_eq!(Solution::level_order(tree![]), vec![vec![]; 0]);
	}
}
