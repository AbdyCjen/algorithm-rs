/**
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * Given inorder and postorder traversal of a tree, construct the binary tree.
 *
 * Note:<br />
 * You may assume that duplicates do not exist in the tree.
 *
 * For example, given
 *
 *
 * inorder = [9,3,15,20,7]
 * postorder = [9,15,7,20,3]
 *
 * Return the following binary tree:
 *
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
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
#[allow(dead_code)]
impl Solution {
	pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		use std::iter::Peekable;
		fn build_tree_inner<I: Iterator<Item = i32>>(
			in_it: &mut Peekable<I>,
			post_it: &mut Peekable<I>,
			stop: Option<i32>,
		) -> Option<Rc<RefCell<TreeNode>>> {
			if stop.as_ref() != in_it.peek() {
				let mut root = TreeNode::new(post_it.next().unwrap());
				root.right = build_tree_inner(in_it, post_it, Some(root.val));
				in_it.next();
				root.left = build_tree_inner(in_it, post_it, stop);
				Some(Rc::new(RefCell::new(root)))
			} else {
				None
			}
		}

		build_tree_inner(
			&mut inorder.into_iter().rev().peekable(),
			&mut postorder.into_iter().rev().peekable(),
			None,
		)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_106() {
		assert_eq!(
			Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
			tree![3, 9, 20, null, null, 15, 7]
		);
	}
}
