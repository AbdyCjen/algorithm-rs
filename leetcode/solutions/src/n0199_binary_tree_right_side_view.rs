/**
 * [199] Binary Tree Right Side View
 *
 * Given a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
 *
 * Example:
 *
 *
 * Input: [1,2,3,null,5,null,4]
 * Output: [1, 3, 4]
 * Explanation:
 *
 *    1            <---
 *  /   \
 * 2     3         <---
 *  \     \
 *   5     4       <---
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
	pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		root.map(|root| {
			let mut root = root.borrow_mut();
			let mut res = vec![root.val];
			res.extend(Self::right_side_view(root.right.take()));
			res.extend(
				Self::right_side_view(root.left.take())
					.into_iter()
					.skip(res.len() - 1),
			);
			res
		})
		.unwrap_or_default()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_199() {
		assert_eq!(
			Solution::right_side_view(tree![1, 2, 3, null, 5, null, 4]),
			vec![1, 3, 4]
		);
	}
}
