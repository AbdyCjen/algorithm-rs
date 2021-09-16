/**
 * [113] Path Sum II
 *
 * Given a binary tree and a sum, find all root-to-leaf paths where each path's sum equals the given sum.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given the below binary tree and sum = 22,
 *
 *
 *       5
 *      / \
 *     4   8
 *    /   / \
 *   11  13  4
 *  /  \    / \
 * 7    2  5   1
 *
 *
 * Return:
 *
 *
 * [
 *    [5,4,11,2],
 *    [5,8,4,5]
 * ]
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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
#[allow(dead_code)]
impl Solution {
	pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
		let mut dq = VecDeque::new();
		if let Some(root) = root {
			dq.push_back((0, vec![], root));
		} else {
			return vec![];
		}
		let mut ans = Vec::new();
		while let Some((mut acc, mut path, cur)) = dq.pop_front() {
			let mut cur = cur.borrow_mut();
			path.push(cur.val);
			acc += cur.val;
			match (cur.left.take(), cur.right.take()) {
				(None, None) => {
					if acc == sum {
						ans.push(path);
					}
				}
				(Some(l), Some(r)) => {
					dq.push_back((acc, path.clone(), l));
					dq.push_back((acc, path, r));
				}
				(Some(n), _) | (_, Some(n)) => dq.push_back((acc, path, n)),
			}
		}

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_113() {
		assert_eq!(
			Solution::path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], 22),
			vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
		)
	}
}
