/**
 * [2476] Closest Nodes Queries in a Binary Search Tree
 *
 * You are given the root of a binary search tree and an array queries of size n consisting of positive integers.
 * Find a 2D array answer of size n where answer[i] = [mini, maxi]:
 *
 *     mini is the largest value in the tree that is smaller than or equal to queries[i]. If a such value does not exist, add -1 instead.
 *     maxi is the smallest value in the tree that is greater than or equal to queries[i]. If a such value does not exist, add -1 instead.
 *
 * Return the array answer.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/28/bstreeedrawioo.png" style="width: 261px; height: 281px;" />
 * Input: root = [6,2,13,1,4,9,15,null,null,null,null,null,null,14], queries = [2,5,16]
 * Output: [[2,2],[4,6],[15,-1]]
 * Explanation: We answer the queries in the following way:
 * - The largest number that is smaller or equal than 2 in the tree is 2, and the smallest number that is greater or equal than 2 is still 2. So the answer for the first query is [2,2].
 * - The largest number that is smaller or equal than 5 in the tree is 4, and the smallest number that is greater or equal than 5 is 6. So the answer for the second query is [4,6].
 * - The largest number that is smaller or equal than 16 in the tree is 15, and the smallest number that is greater or equal than 16 does not exist. So the answer for the third query is [15,-1].
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/28/bstttreee.png" style="width: 101px; height: 121px;" />
 * Input: root = [4,null,9], queries = [3]
 * Output: [[-1,4]]
 * Explanation: The largest number that is smaller or equal to 3 in the tree does not exist, and the smallest number that is greater or equal to 3 is 4. So the answer for the query is [-1,4].
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [2, 10^5].
 *     1 <= Node.val <= 10^6
 *     n == queries.length
 *     1 <= n <= 10^5
 *     1 <= queries[i] <= 10^6
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
	pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
		let mut nums = Vec::new();
		Self::to_vec(root, &mut nums);
		let n = nums.len();
		let mut ans = Vec::new();
		for q in queries {
			match nums.binary_search(&q) {
				Ok(_) => ans.push(vec![q, q]),
				Err(0) => ans.push(vec![-1, nums[0]]),
				Err(i) if i == n => ans.push(vec![nums[i - 1], -1]),
				Err(i) => ans.push(vec![nums[i - 1], nums[i]]),
			}
		}
		ans
	}

	fn compress(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> { None }

	fn straighten(
		root: Option<Rc<RefCell<TreeNode>>>,
		tail: Option<Rc<RefCell<TreeNode>>>,
	) -> Option<Rc<RefCell<TreeNode>>> {
		if let Some(root) = root {
			let mut root_mut = root.borrow_mut();
			root_mut.right = Self::straighten(root_mut.right.take(), tail);
			let left = root_mut.left.take();
			drop(root_mut);
			Self::straighten(left, Some(root))
		} else {
			tail
		}
	}

	fn to_vec(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
		if let Some(root) = root {
			let mut root = root.borrow_mut();

			Self::to_vec(root.left.take(), v);
			v.push(root.val);
			Self::to_vec(root.right.take(), v);
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_2476() {
		assert_eq!(
			Solution::closest_nodes(
				tree![6, 2, 13, 1, 4, 9, 15, null, null, null, null, null, null, 14],
				vec![2, 5, 16]
			),
			matrix![[2, 2], [4, 6], [15, -1]]
		);
	}
}
