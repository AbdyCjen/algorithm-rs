/**
 * [104] Maximum Depth of Binary Tree
 *
 * Given the root of a binary tree, return its maximum depth.
 * A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/tmp-tree.jpg" style="width: 400px; height: 277px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 3
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1,null,2]
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 10^4].
 *     -100 <= Node.val <= 100
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
	pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::solve(&root) 
	}

	fn solve(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
		match root {
			None => 0,
			Some(root) => {
				let root = root.borrow();
				1 + Self::solve(&root.left).max(Self::solve(&root.right))
			}
		}
	}
}

// submission codes end
