/**
 * [543] Diameter of Binary Tree
 *
 *
 * Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
 *
 *
 *
 * Example:<br />
 * Given a binary tree <br />
 *
 *           1
 *          / \
 *         2   3
 *        / \     
 *       4   5    
 *
 *
 *
 * Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
 *
 *
 * Note:
 * The length of path between two nodes is represented by the number of edges between them.
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
	pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::height_and_diameter(root).1
	}

	fn height_and_diameter(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
		match root {
			Some(root) => {
				let (hl, dl) = Self::height_and_diameter(root.borrow().left.clone());
				let (hr, dr) = Self::height_and_diameter(root.borrow().right.clone());
				(
					std::cmp::max(hl, hr) + 1,
					*[dl, dr, hl + hr].iter().max().unwrap(),
				)
			}
			None => (0, 0),
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_543() {
		assert_eq!(Solution::diameter_of_binary_tree(tree![1, 2, 3, 4, 5]), 3);
		assert_eq!(Solution::diameter_of_binary_tree(tree![1]), 0);
		assert_eq!(Solution::diameter_of_binary_tree(tree![1, 2]), 1);
	}
}
