/**
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
 * Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
 * Output: [3,9,20,null,null,15,7]
 *
 * Example 2:
 *
 * Input: preorder = [-1], inorder = [-1]
 * Output: [-1]
 *
 *  
 * Constraints:
 *
 *     1 <= preorder.length <= 3000
 *     inorder.length == preorder.length
 *     -3000 <= preorder[i], inorder[i] <= 3000
 *     preorder and inorder consist of unique values.
 *     Each value of inorder also appears in preorder.
 *     preorder is guaranteed to be the preorder traversal of the tree.
 *     inorder is guaranteed to be the inorder traversal of the tree.
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
#[allow(dead_code)]
impl Solution {
	pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		use std::iter::Peekable;
		fn build_tree_inner<I: Iterator<Item = i32>>(
			pre_it: &mut Peekable<I>,
			in_it: &mut Peekable<I>,
			stop: Option<i32>,
		) -> Option<Rc<RefCell<TreeNode>>> {
			if in_it.peek() != stop.as_ref() {
				let mut root = TreeNode::new(pre_it.next().unwrap());
				root.left = build_tree_inner(pre_it, in_it, Some(root.val));
				in_it.next();
				root.right = build_tree_inner(pre_it, in_it, stop);
				Some(Rc::new(RefCell::new(root)))
			} else {
				None
			}
		}
		let mut pre_it = preorder.into_iter().peekable();
		let mut in_it = inorder.into_iter().peekable();
		build_tree_inner(&mut pre_it, &mut in_it, None)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_105() {
		assert_eq!(
			Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
			tree![3, 9, 20, null, null, 15, 7]
		);
		assert_eq!(Solution::build_tree(vec![-1], vec![-1]), tree![-1]);
	}
}
