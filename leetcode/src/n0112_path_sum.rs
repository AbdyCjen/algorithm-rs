/**
 * [112] Path Sum
 *
 * Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given the below binary tree and sum = 22,
 *
 *
 *       5
 *      / \
 *     4   8
 *    /   / \
 *   11  13  4
 *  /  \      \
 * 7    2      1
 *
 *
 * return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
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
use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
		let root = match root {
			None => return false,
			Some(node) => node,
		};
		let root = root.borrow();
		if let (None, None) = (&root.left, &root.right) {
			return sum == root.val;
		}
		return Solution::has_path_sum(root.left.clone(), sum - root.val)
			|| Solution::has_path_sum(root.right.clone(), sum - root.val);
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_112() {
		assert_eq!(
			Solution::has_path_sum(
				tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
				22
			),
			true
		);
	}
}
