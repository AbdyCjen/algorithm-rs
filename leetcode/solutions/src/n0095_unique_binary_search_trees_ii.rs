/**
 * [95] Unique Binary Search Trees II
 *
 * Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 8
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> { Self::solve(1, n) }
	fn solve(begin: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
		if begin > end {
			return vec![None];
		}
		let mut ans = vec![];
		for i in begin..=end {
			for l in Self::solve(begin, i - 1) {
				for r in Self::solve(i + 1, end) {
					ans.push(Some(Rc::new(RefCell::new(TreeNode {
						val: i,
						left: l.clone(),
						right: r,
					}))));
				}
			}
		}
		ans
	}
}

// submission codes end
