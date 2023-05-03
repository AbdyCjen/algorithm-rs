/**
 * [958] Check Completeness of a Binary Tree
 *
 * Given the root of a binary tree, determine if it is a complete binary tree.
 * In a <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">complete binary tree</a>, every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-1.png" style="width: 180px; height: 145px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: true
 * Explanation: Every level before the last is full (ie. levels with node-values {1} and {2, 3}), and all nodes in the last level ({4, 5, 6}) are as far left as possible.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-2.png" style="width: 200px; height: 145px;" />
 * Input: root = [1,2,3,4,5,null,7]
 * Output: false
 * Explanation: The node with value 7 isn't as far left as possible.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 100].
 *     1 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn is_complete_tree1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		let mut dq = std::collections::VecDeque::new();
		dq.push_back(root);
		while let Some(Some(no)) = dq.pop_front() {
			let mut no = no.borrow_mut();
			dq.push_back(no.left.take());
			dq.push_back(no.right.take());
		}
		dq.into_iter().flatten().next().is_none()
	}
}
