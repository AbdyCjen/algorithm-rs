/**
 * [144] Binary Tree Preorder Traversal
 *
 * Given the root of a binary tree, return the preorder traversal of its nodes' values.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_1.jpg" style="width: 125px; height: 200px;" />
 * Input: root = [1,null,2,3]
 * Output: [1,2,3]
 *
 * <strong class="example">Example 2:
 *
 * Input: root = []
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 100].
 *     -100 <= Node.val <= 100
 *
 *  
 * Follow up: Recursive solution is trivial, could you do it iteratively?
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
	pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let root = match root {
			None => return Vec::new(),
			Some(r) => r,
		};
		let mut st = vec![root];
		let mut ans = vec![];
		while let Some(r) = st.pop() {
			let mut cur = r.borrow_mut();
			ans.push(cur.val);
			if let Some(right) = cur.right.take() {
				st.push(right);
			}
			if let Some(left) = cur.left.take() {
				st.push(left);
			}
		}
		ans
	}
}
