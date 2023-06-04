/**
 * [783] Minimum Distance Between BST Nodes
 *
 * Given the root of a Binary Search Tree (BST), return the minimum difference between the values of any two different nodes in the tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst1.jpg" style="width: 292px; height: 301px;" />
 * Input: root = [4,2,6,1,3]
 * Output: 1
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst2.jpg" style="width: 282px; height: 301px;" />
 * Input: root = [1,0,48,null,null,12,49]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [2, 100].
 *     0 <= Node.val <= 10^5
 *
 *  
 * Note: This question is the same as 530: <a href="https://leetcode.com/problems/minimum-absolute-difference-in-bst/" target="_blank">https://leetcode.com/problems/minimum-absolute-difference-in-bst/</a>
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
	pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut v = vec![];
		Self::collect(&root, &mut v);
		v.windows(2).map(|w| w[1] - w[0]).min().unwrap()
	}

	fn collect(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
		if let Some(root) = root {
			let root = root.borrow();
			Self::collect(&root.left, v);
			v.push(root.val);
			Self::collect(&root.right, v);
		}
	}
}

// submission codes end
