/**
 * [515] Find Largest Value in Each Tree Row
 *
 * Given the root of a binary tree, return an array of the largest value in each row of the tree (0-indexed).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/21/largest_e1.jpg" style="width: 300px; height: 172px;" />
 * Input: root = [1,3,2,5,3,null,9]
 * Output: [1,3,9]
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1,2,3]
 * Output: [1,3]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree will be in the range [0, 10^4].
 *     -2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut ans = vec![];
		let mut dq = vec![];
		match root {
			Some(root) => dq.push(root),
			None => return ans,
		};
		while !dq.is_empty() {
			let mut a = i32::MIN;
			for no in std::mem::take(&mut dq) {
				let mut no = no.borrow_mut();
				a = a.max(no.val);
				if let Some(l) = no.left.take() {
					dq.push(l);
				}
				if let Some(r) = no.right.take() {
					dq.push(r);
				}
			}
			ans.push(a);
		}
		ans
	}
}

// submission codes end
