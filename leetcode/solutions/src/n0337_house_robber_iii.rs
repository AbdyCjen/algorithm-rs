/**
 * [337] House Robber III
 *
 * The thief has found himself a new place for his thievery again. There is only one entrance to this area, called root.
 * Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that all houses in this place form a binary tree. It will automatically contact the police if two directly-linked houses were broken into on the same night.
 * Given the root of the binary tree, return the maximum amount of money the thief can rob without alerting the police.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/rob1-tree.jpg" style="width: 277px; height: 293px;" />
 * Input: root = [3,2,3,null,3,null,1]
 * Output: 7
 * Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/rob2-tree.jpg" style="width: 357px; height: 293px;" />
 * Input: root = [3,4,5,1,3,null,1]
 * Output: 9
 * Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 10^4].
 *     0 <= Node.val <= 10^4
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
	pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let root = match &root {
			None => return 0,
			Some(root) => root.borrow(),
		};
		Self::solve(&root).0
	}

	fn solve(root: &TreeNode) -> (i32, i32) {
		let (mut with, mut without) = (root.val, 0);
		for no in [&root.left, &root.right].iter().copied().flatten() {
			let nv = Self::solve(&no.borrow());
			with += nv.1;
			without += nv.0;
		}

		(with.max(without), without)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_337() {}
}
