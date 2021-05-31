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

use std::cell::RefCell;
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
use std::rc::Rc;
#[allow(dead_code)]
impl Solution {
	pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		Self::build(&inorder, &postorder)
	}

	// copy from the example, mine is too ugly
	fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
		//dbg!(inorder,postorder);
		let root_val = postorder.last()?;
		let slc_ind = inorder.iter().position(|x| x == root_val).unwrap();

		Some(Rc::new(RefCell::new(TreeNode {
			val: *root_val,
			left: Self::build(&inorder[..slc_ind], &postorder[..slc_ind]),
			right: Self::build(
				&inorder[slc_ind + 1..],
				&postorder[slc_ind..postorder.len() - 1],
			),
		})))
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
