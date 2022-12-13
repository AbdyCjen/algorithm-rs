/**
 * [124] Binary Tree Maximum Path Sum
 *
 * Given a non-empty binary tree, find the maximum path sum.
 *
 * For this problem, a path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections. The path must contain at least one node and does not need to go through the root.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3]
 *
 *        1
 *       / \
 *      2   3
 *
 * Output: 6
 *
 *
 * Example 2:
 *
 *
 * Input: [-10,9,20,null,null,15,7]
 *
 *    -10
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 * Output: 42
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
use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		root.map(|r| Self::solve(&r.borrow()).1).unwrap_or(0)
	}
	fn solve(root: &TreeNode) -> (i32, i32) {
		let (mut sum, mut ans, mut cand) = (0, root.val, root.val);
		for no in [&root.left, &root.right].iter().copied().flatten() {
			let (s, a) = Self::solve(&no.borrow());
			sum = sum.max(s);
			cand += 0.max(s);
			ans = ans.max(a);
		}
		(sum + root.val, ans.max(cand))
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_124() {
		assert_eq!(Solution::max_path_sum(tree![1, 2, 3]), 6);
		assert_eq!(Solution::max_path_sum(tree![2, -1]), 2);
		assert_eq!(Solution::max_path_sum(tree![-3]), -3);
		assert_eq!(
			Solution::max_path_sum(tree![-10, 9, 20, null, null, 15, 7]),
			42
		);
		assert_eq!(
			Solution::max_path_sum(tree![
				9, 6, -3, null, null, -6, 2, null, null, 2, null, -6, -6, -6
			]),
			16
		);
	}
}
