/**
 * [1161] Maximum Level Sum of a Binary Tree
 *
 * Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.
 * Return the smallest level x such that the sum of all the values of nodes at level x is maximal.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/capture.JPG" style="width: 200px; height: 175px;" />
 * Input: root = [1,7,0,7,-8,null,null]
 * Output: 2
 * Explanation:
 * Level 1 sum = 1.
 * Level 2 sum = 7 + 0 = 7.
 * Level 3 sum = 7 + -8 = -1.
 * So we return the level with the maximum sum which is level 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [989,null,10250,98693,-89388,null,null,null,-32127]
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 10^4].
 *     -10^5 <= Node.val <= 10^5
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
	pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut sum = vec![];
		Self::solve(&root, 0, &mut sum);
		sum.into_iter()
			.zip(1..)
			.fold(
				(0, i32::MIN),
				|(a, s), (n, i)| if n > s { (i, n) } else { (a, s) },
			)
			.0
	}

	fn solve(root: &Option<Rc<RefCell<TreeNode>>>, lev: u32, sum: &mut Vec<i32>) {
		let root = match root {
			Some(r) => r.borrow(),
			_ => return,
		};
		if sum.len() <= lev as usize {
			sum.push(0);
		}
		sum[lev as usize] += root.val;
		Self::solve(&root.left, lev + 1, sum);
		Self::solve(&root.right, lev + 1, sum);
	}
}

// submission codes end
