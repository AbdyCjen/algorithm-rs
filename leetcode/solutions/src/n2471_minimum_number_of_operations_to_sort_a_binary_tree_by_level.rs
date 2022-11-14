/**
 * [2471] Minimum Number of Operations to Sort a Binary Tree by Level
 *
 * You are given the root of a binary tree with unique values.
 * In one operation, you can choose any two nodes at the same level and swap their values.
 * Return the minimum number of operations needed to make the values at each level sorted in a strictly increasing order.
 * The level of a node is the number of edges along the path between it and the root node.
 *  
 * <strong class="example">Example 1:
 * <img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174006-2.png" style="width: 500px; height: 324px;" />
 * Input: root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
 * Output: 3
 * Explanation:
 * - Swap 4 and 3. The 2^nd level becomes [3,4].
 * - Swap 7 and 5. The 3^rd level becomes [5,6,8,7].
 * - Swap 8 and 7. The 3^rd level becomes [5,6,7,8].
 * We used 3 operations so return 3.
 * It can be proven that 3 is the minimum number of operations needed.
 *
 * <strong class="example">Example 2:
 * <img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174026-3.png" style="width: 400px; height: 303px;" />
 * Input: root = [1,3,2,7,6,5,4]
 * Output: 3
 * Explanation:
 * - Swap 3 and 2. The 2^nd level becomes [2,3].
 * - Swap 7 and 4. The 3^rd level becomes [4,6,5,7].
 * - Swap 6 and 5. The 3^rd level becomes [4,5,6,7].
 * We used 3 operations so return 3.
 * It can be proven that 3 is the minimum number of operations needed.
 *
 * <strong class="example">Example 3:
 * <img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174052-4.png" style="width: 400px; height: 274px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: 0
 * Explanation: Each level is already sorted in increasing order so return 0.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 10^5].
 *     1 <= Node.val <= 10^5
 *     All the values of the tree are unique.
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
	pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let root = if let Some(root) = root {
			root
		} else {
			return 0;
		};
		let mut dq = vec![root];
		let mut levs: Vec<Vec<_>> = vec![];
		while !dq.is_empty() {
			let cur_lev = dq.split_off(0);
			levs.push(cur_lev.iter().map(|no| no.borrow().val).collect());
			for no in cur_lev {
				let mut no = no.borrow_mut();
				if let Some(l) = no.left.take() {
					dq.push(l);
				}
				if let Some(r) = no.right.take() {
					dq.push(r);
				}
			}
		}
		let levs_sort = levs
			.iter()
			.cloned()
			.map(|mut v| {
				v.sort_unstable();
				v
			})
			.collect::<Vec<_>>();
		let mut ans = 0;
		for (ov, sv) in levs.iter_mut().zip(&levs_sort) {
			for i in 0..sv.len() {
				while ov[i] != sv[i] {
					let t = sv.binary_search(&ov[i]).unwrap();
					ov.swap(i, t);
					ans += 1;
				}
			}
		}
		ans
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::tree::*;

	#[test]
	fn test_2471() {
		assert_eq!(
			Solution::minimum_operations(tree![
				1, 4, 3, 7, 6, 8, 5, null, null, null, null, 9, null, 10
			]),
			3
		);
		assert_eq!(Solution::minimum_operations(tree![1, 3, 2, 7, 6, 5, 4]), 3);
		assert_eq!(Solution::minimum_operations(tree![1, 2, 3, 4, 5, 6]), 0);
	}
}
