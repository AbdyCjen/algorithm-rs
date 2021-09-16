/**
 * [662] Maximum Width of Binary Tree
 *
 * Given a binary tree, write a function to get the maximum width of the given tree. The maximum width of a tree is the maximum width among all levels.
 * The width of one level is defined as the length between the end-nodes (the leftmost and right most non-null nodes in the level, where the null nodes between the end-nodes are also counted into the length calculation.
 * It is guaranteed that the answer will in the range of 32-bit signed integer.
 * Example 1:
 *
 * Input:
 *            1
 *          /   \
 *         3     2
 *        / \     \  
 *       5   3     9
 * Output: 4
 * Explanation: The maximum width existing in the third level with the length 4 (5,3,null,9).
 *
 * Example 2:
 *
 * Input:
 *           1
 *          /  
 *         3    
 *        / \       
 *       5   3     
 * Output: 2
 * Explanation: The maximum width existing in the third level with the length 2 (5,3).
 *
 * Example 3:
 *
 * Input:
 *           1
 *          / \
 *         3   2
 *        /        
 *       5      
 * Output: 2
 * Explanation: The maximum width existing in the second level with the length 2 (3,2).
 *
 * Example 4:
 *
 * Input:
 *           1
 *          / \
 *         3   2
 *        /     \  
 *       5       9
 *      /         \
 *     6           7
 * Output: 8
 * Explanation:The maximum width existing in the fourth level with the length 8 (6,null,null,null,null,null,null,7).
 *
 *  
 * Constraints:
 *
 *     The given binary tree will have between 1 and 3000 nodes.
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
	pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut dq = VecDeque::new();
		dq.push_back((root.unwrap(), 1));
		let mut ans = 0_u64;
		while !dq.is_empty() {
			ans = ans.max(dq.back().unwrap().1 - dq.front().unwrap().1 + 1);
			for (no, idx) in dq.split_off(0) {
				let mut no = no.borrow_mut();
				if let Some(l) = no.left.take() {
					dq.push_back((l, idx * 2));
				}
				if let Some(r) = no.right.take() {
					dq.push_back((r, idx * 2 + 1));
				}
			}
		}
		ans as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_662() {
		assert_eq!(
			Solution::width_of_binary_tree(tree![1, 3, 2, 5, 3, null, 9]),
			4
		);
		assert_eq!(Solution::width_of_binary_tree(tree![1, 3, null, 5, 3]), 2);
		assert_eq!(Solution::width_of_binary_tree(tree![1, 3, 2, 5]), 2);
		assert_eq!(
			Solution::width_of_binary_tree(tree![1, 3, 2, 5, null, null, 9, 6, null, null, 7]),
			8
		);
	}
}
