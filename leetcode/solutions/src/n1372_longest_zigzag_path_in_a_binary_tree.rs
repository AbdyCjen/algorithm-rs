/**
 * [1372] Longest ZigZag Path in a Binary Tree
 *
 * You are given the root of a binary tree.
 * A ZigZag path for a binary tree is defined as follow:
 *
 *     Choose any node in the binary tree and a direction (right or left).
 *     If the current direction is right, move to the right child of the current node; otherwise, move to the left child.
 *     Change the direction from right to left or from left to right.
 *     Repeat the second and third steps until you can't move in the tree.
 *
 * Zigzag length is defined as the number of nodes visited - 1. (A single node has a length of 0).
 * Return the longest ZigZag path contained in that tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/22/sample_1_1702.png" style="width: 221px; height: 383px;" />
 * Input: root = [1,null,1,1,1,null,null,1,1,null,1,null,null,null,1,null,1]
 * Output: 3
 * Explanation: Longest ZigZag path in blue nodes (right -> left -> right).
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/22/sample_2_1702.png" style="width: 157px; height: 329px;" />
 * Input: root = [1,1,1,null,1,null,null,1,1,null,1]
 * Output: 4
 * Explanation: Longest ZigZag path in blue nodes (left -> right -> left -> right).
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 5 * 10^4].
 *     1 <= Node.val <= 100
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
	pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::solve(&root, 0)[1] - 1
	}

	fn solve(root: &Option<Rc<RefCell<TreeNode>>>, idx: u8) -> [i32; 2] {
		match root {
			None => [0; 2],
			Some(root) => {
				let root = root.borrow();
				let mut ans = [0; 3];
				for (i, ch) in (0..).zip([&root.left, &root.right]) {
					let cnt = Self::solve(ch, i ^ 1);
					ans[i as usize] = cnt[0] + 1;
					ans[2] = ans[2].max(cnt[1]).max(cnt[0] + 1);
				}
				[ans[idx as usize], ans[2]]
			}
		}
	}
}

// submission codes end
