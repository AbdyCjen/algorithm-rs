/**
 * [530] Minimum Absolute Difference in BST
 *
 * Given a binary search tree with non-negative values, find the minimum <a href="https://en.wikipedia.org/wiki/Absolute_difference">absolute difference</a> between values of any two nodes.
 * Example:
 *
 * Input:
 *    1
 *     \
 *      3
 *     /
 *    2
 * Output:
 * 1
 * Explanation:
 * The minimum absolute difference is 1, which is the difference between 2 and 1 (or between 2 and 3).
 *
 *  
 * Note:
 *
 * There are at least two nodes in this BST.
 * This question is the same as 783: <a href="https://leetcode.com/problems/minimum-distance-between-bst-nodes/">https://leetcode.com/problems/minimum-distance-between-bst-nodes/</a>
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
	pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::solve(&root, root.as_ref().unwrap().borrow().val - 200_000).1
	}
	fn solve(root: &Option<Rc<RefCell<TreeNode>>>, prv: i32) -> (i32, i32) {
		match root {
			Some(root) => {
				let root = root.borrow();
				let (prv, mut ans) = Self::solve(&root.left, prv);
				ans = ans.min(root.val - prv);
				let (prv, can) = Self::solve(&root.right, root.val);
				(prv, can.min(ans))
			}
			_ => (prv, i32::MAX),
		}
	}

	pub fn get_minimum_difference1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut st = Vec::new();
		let mut o = root;
		while let Some(lo) = o.take() {
			o = lo.borrow().left.clone();
			st.push(lo);
		}

		let mut pre = std::i32::MIN / 2;
		let mut res = std::i32::MAX;
		while let Some(o) = st.pop() {
			let o = o.borrow();
			res = std::cmp::min(res, o.val - pre);
			pre = o.val;

			let mut o = o.right.clone();
			while let Some(lo) = o.take() {
				o = lo.borrow().left.clone();
				st.push(lo);
			}
		}
		res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_530() {
		assert_eq!(Solution::get_minimum_difference(tree![1, null, 3, 2]), 1);
		assert_eq!(Solution::get_minimum_difference(tree![5, 4, 7]), 1);
	}
}
