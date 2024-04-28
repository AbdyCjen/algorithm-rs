/**
 * [437] Path Sum III
 *
 * Given the root of a binary tree and an integer targetSum, return the number of paths where the sum of the values along the path equals targetSum.
 * The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/pathsum3-1-tree.jpg" style="width: 450px; height: 386px;" />
 * Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
 * Output: 3
 * Explanation: The paths that sum to 8 are shown.
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 1000].
 *     -10^9 <= Node.val <= 10^9
 *     -1000 <= targetSum <= 1000
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
use std::{cell::RefCell, collections::*, rc::Rc};
impl Solution {
	pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
		Self::solve(&root, target_sum, 0, &mut HashMap::from_iter([(0, 1)]))
	}
	fn solve(
		root: &Option<Rc<RefCell<TreeNode>>>,
		tsum: i32,
		mut psum: i64,
		sums: &mut HashMap<i64, i32>,
	) -> i32 {
		match root {
			Some(r) => {
				let r = r.borrow();
				psum += r.val as i64;
				let mut ans = *sums.get(&(psum - tsum as i64)).unwrap_or(&0);
				*sums.entry(psum).or_insert(0) += 1;
				ans += Self::solve(&r.left, tsum, psum, sums)
					+ Self::solve(&r.right, tsum, psum, sums);
				*sums.get_mut(&psum).unwrap() -= 1;
				ans
			}
			None => 0,
		}
	}
}

// submission codes end
