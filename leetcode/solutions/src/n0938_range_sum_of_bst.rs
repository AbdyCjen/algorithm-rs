/**
 * [938] Range Sum of BST
 *
 * Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes with a value in the inclusive range [low, high].
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst1.jpg" style="width: 400px; height: 222px;" />
 * Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
 * Output: 32
 * Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst2.jpg" style="width: 400px; height: 335px;" />
 * Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
 * Output: 23
 * Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 2 * 10^4].
 *     1 <= Node.val <= 10^5
 *     1 <= low <= high <= 10^5
 *     All Node.val are unique.
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
	pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, h: i32) -> i32 {
		Self::solve(&root, l, h)
	}

	fn solve(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, h: i32) -> i32 {
		match root {
			Some(root) => {
				let root = root.borrow();
				use std::cmp::Ordering;
				match root.val.cmp(&l) {
					Ordering::Less => Self::solve(&root.right, l, h),
					Ordering::Equal => root.val + Self::solve(&root.right, l, h),
					Ordering::Greater => match root.val.cmp(&h) {
						Ordering::Less => {
							Self::solve(&root.left, l, h)
								+ Self::solve(&root.right, l, h) + root.val
						}
						Ordering::Equal => root.val + Self::solve(&root.left, l, h),
						Ordering::Greater => Self::solve(&root.left, l, h),
					},
				}
			}
			_ => 0,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_938() {}
}
