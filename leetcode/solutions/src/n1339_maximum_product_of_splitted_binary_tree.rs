/**
 * [1339] Maximum Product of Splitted Binary Tree
 *
 * Given the root of a binary tree, split the binary tree into two subtrees by removing one edge such that the product of the sums of the subtrees is maximized.
 * Return the maximum product of the sums of the two subtrees. Since the answer may be too large, return it modulo 10^9 + 7.
 * Note that you need to maximize the answer before taking the mod and not after taking it.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/sample_1_1699.png" style="width: 500px; height: 167px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: 110
 * Explanation: Remove the red edge and get 2 binary trees with sum 11 and 10. Their product is 110 (11*10)
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/sample_2_1699.png" style="width: 500px; height: 211px;" />
 * Input: root = [1,null,2,3,4,null,null,5,6]
 * Output: 90
 * Explanation: Remove the red edge and get 2 binary trees with sum 15 and 6.Their product is 90 (15*6)
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [2, 5 * 10^4].
 *     1 <= Node.val <= 10^4
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
const MO: i64 = 1e9 as i64 + 7;
impl Solution {
	pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let s = Self::sum(&root);
		(Self::solve(&root.as_ref().unwrap().borrow(), s).1 % MO) as i32
	}

	fn sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
		match root.as_ref().map(|r| r.borrow()) {
			Some(root) => [&root.left, &root.right]
				.iter()
				.fold(root.val, |s, no| s + Self::sum(no)),
			_ => 0,
		}
	}

	fn solve(root: &TreeNode, tot: i32) -> (i32, i64) {
		let (mut sum, mut ans) = (root.val, 0);
		for no in [&root.left, &root.right].iter().copied().flatten() {
			let nv = Self::solve(&no.borrow(), tot);
			let cand = (tot - nv.0) as i64 * nv.0 as i64;
			ans = ans.max(cand).max(nv.1);
			sum += nv.0;
		}
		(sum, ans)
	}
}

// submission codes end

#[cfg(test)]
mod tests {

	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_1339() {
		assert_eq!(Solution::max_product(tree![1, 2, 3, 4, 5, 6]), 110);
		assert_eq!(
			Solution::max_product(tree![1, null, 2, 3, 4, null, null, 5, 6]),
			90
		);
	}
}
