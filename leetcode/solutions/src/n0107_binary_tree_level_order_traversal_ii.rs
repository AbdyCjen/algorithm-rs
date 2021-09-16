/**
 * [107] Binary Tree Level Order Traversal II
 *
 * Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
 *
 *
 * For example:<br />
 * Given binary tree [3,9,20,null,null,15,7],<br />
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 *
 * return its bottom-up level order traversal as:<br />
 *
 * [
 *   [15,7],
 *   [9,20],
 *   [3]
 * ]
 *
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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
#[allow(dead_code)]
impl Solution {
	pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
		let mut dq = VecDeque::new();
		if let Some(root) = root {
			dq.push_back(root);
		} else {
			return vec![];
		}
		let mut ans = VecDeque::new();
		while !dq.is_empty() {
			let mut lev = Vec::new();
			for cur in dq.split_off(0) {
				let mut cur = cur.borrow_mut();
				lev.push(cur.val);
				if let Some(l) = cur.left.take() {
					dq.push_back(l);
				}
				if let Some(r) = cur.right.take() {
					dq.push_back(r);
				}
			}
			ans.push_front(lev);
		}
		ans.into()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_107() {
		assert_eq!(
			Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
			vec![vec![15, 7], vec![9, 20], vec![3]]
		);
	}
}
