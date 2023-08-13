/**
 * [894] All Possible Full Binary Trees
 *
 * Given an integer n, return a list of all possible full binary trees with n nodes. Each node of each tree in the answer must have Node.val == 0.
 * Each element of the answer is the root node of one possible tree. You may return the final list of trees in any order.
 * A full binary tree is a binary tree where each node has exactly 0 or 2 children.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/22/fivetrees.png" style="width: 700px; height: 400px;" />
 * Input: n = 7
 * Output: [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 3
 * Output: [[0,0,0]]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 20
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
		let mut dp = vec![vec![]; n as usize + 1];
		dp[1].push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
		for i in (3..=n).step_by(2) {
			let mut row = vec![];
			for j in (1..i).step_by(2) {
				for l in &dp[j as usize] {
					for r in &dp[(i - j - 1) as usize] {
						row.push(Some(Rc::new(RefCell::new(TreeNode {
							val: 0,
							left: l.clone(),
							right: r.clone(),
						}))));
					}
				}
			}
			dp[i as usize] = row;
		}
		dp.pop().unwrap()
	}
}

// submission codes end
