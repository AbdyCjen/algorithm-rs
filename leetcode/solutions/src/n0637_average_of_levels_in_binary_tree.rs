/**
 * [637] Average of Levels in Binary Tree
 *
 * Given a non-empty binary tree, return the average value of the nodes on each level in the form of an array.
 *
 * Example 1:<br />
 *
 * Input:
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * Output: [3, 14.5, 11]
 * Explanation:
 * The average value of nodes on level 0 is 3,  on level 1 is 14.5, and on level 2 is 11. Hence return [3, 14.5, 11].
 *
 *
 *
 * Note:<br>
 * <ol>
 * The range of node's value is in the range of 32-bit signed integer.
 * </ol>
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
	pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
		let mut dq = VecDeque::new();
		if let Some(root) = root {
			dq.push_back(root);
		}

		let mut ans = Vec::new();
		let (mut cnt, mut sum) = (0, 0_i64);
		while !dq.is_empty() {
			for cur in dq.split_off(0) {
				let mut cur = cur.borrow_mut();
				if let Some(l) = cur.left.take() {
					dq.push_back(l);
				}
				if let Some(r) = cur.right.take() {
					dq.push_back(r);
				}
				cnt += 1;
				sum += cur.val as i64;
			}
			ans.push(sum as f64 / cnt as f64);
			cnt = 0;
			sum = 0;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_637() {
		assert_eq!(
			Solution::average_of_levels(tree![3, 9, 20, 15, 7]),
			vec![3_f64, 14.5, 11_f64]
		);
		assert_eq!(
			Solution::average_of_levels(tree![2147483647, 2147483647, 2147483647]),
			vec![2147483647.00000, 2147483647.00000]
		);
	}
}
