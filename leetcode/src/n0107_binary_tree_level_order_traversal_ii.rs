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
use super::util::tree::{to_tree, TreeNode};

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
		if root.is_none() {
			return Vec::new();
		}
		let mut dq = VecDeque::new();
		dq.push_back(root);
		dq.push_back(None);
		let mut res = Vec::new();
		let mut lev = Vec::new();
		while let Some(o) = dq.pop_front() {
			match o {
				None => {
					res.push(lev);
					lev = Vec::new();
					if !dq.is_empty() {
						dq.push_back(None);
					}
				}
				Some(o) => {
					let o = o.borrow();
					lev.push(o.val);
					if o.left.is_some() {
						dq.push_back(o.left.clone())
					};
					if o.right.is_some() {
						dq.push_back(o.right.clone())
					};
				}
			}
		}
		res.reverse();
		res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_107() {
		assert_eq!(
			Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
			vec![vec![15, 7], vec![9, 20], vec![3]]
		);
	}
}
