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
impl Solution {
	pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		Self::solve(&inorder, &postorder)
	}

	fn solve(inor: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
		match post {
			[post @ .., val] => {
				let ox = inor.iter().position(|i| i == val).unwrap();
				Some(Rc::new(RefCell::new(TreeNode {
					val: *val,
					left: Self::solve(&inor[..ox], &post[..ox]),
					right: Self::solve(&inor[ox + 1..], &post[ox..]),
				})))
			}
			_ => None,
		}
	}

	pub fn build_tree_01(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		Self::build(&inorder, &postorder, None).1
	}
	fn build(
		inor: &[i32],
		post: &[i32],
		stop: Option<&i32>,
	) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
		match post {
			[post @ .., val] if stop != inor.last() => {
				let (rc, right) = Self::build(inor, post, Some(val));
				let (inor, post) = (&inor[..inor.len() - rc - 1], &post[..post.len() - rc]);
				let (lc, left) = Self::build(inor, post, stop);
				(
					lc + rc + 1,
					Some(Rc::new(RefCell::new(TreeNode {
						left,
						right,
						val: *val,
					}))),
				)
			}
			_ => (0, None),
		}
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
