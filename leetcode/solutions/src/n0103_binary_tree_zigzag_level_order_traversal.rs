/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[3],[20,9],[15,7]]
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1]
 * Output: [[1]]
 *
 * <strong class="example">Example 3:
 *
 * Input: root = []
 * Output: []
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 2000].
 *     -100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
		let mut st = root.into_iter().collect::<Vec<_>>();
		let mut ans = vec![];
		while !st.is_empty() {
			if ans.len() % 2 == 0 {
				ans.push(st.iter().map(|n| n.borrow().val).collect());
			} else {
				ans.push(st.iter().rev().map(|n| n.borrow().val).collect());
			}
			for no in std::mem::take(&mut st) {
				let mut no = no.borrow_mut();
				if let Some(l) = no.left.take() {
					st.push(l);
				}
				if let Some(r) = no.right.take() {
					st.push(r);
				}
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
	fn test_103() {
		assert_eq!(
			Solution::zigzag_level_order(tree![1, 2, 3, 4, null, null, 5]),
			matrix![[1], [3, 2], [4, 5]]
		);
		assert_eq!(
			Solution::zigzag_level_order(tree![3, 9, 20, null, null, 15, 7]),
			matrix![[3], [20, 9], [15, 7]]
		);
	}
}
