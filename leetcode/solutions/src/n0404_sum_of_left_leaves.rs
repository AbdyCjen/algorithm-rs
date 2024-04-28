/**
 * [404] Sum of Left Leaves
 *
 * Given the root of a binary tree, return the sum of all left leaves.
 * A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/leftsum-tree.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 24
 * Explanation: There are two left leaves in the binary tree, with values 9 and 15 respectively.
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 1000].
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
impl Solution {
	pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::solve(&root.unwrap().borrow(), false)
	}
	fn solve(root: &TreeNode, is_left: bool) -> i32 {
		match (&root.left, &root.right) {
			(None, None) => root.val * (is_left as i32),
			(Some(l), None) => Self::solve(&l.borrow(), true),
			(None, Some(r)) => Self::solve(&r.borrow(), false),
			(Some(l), Some(r)) => Self::solve(&l.borrow(), true) + Self::solve(&r.borrow(), false),
		}
	}
}

// submission codes end
