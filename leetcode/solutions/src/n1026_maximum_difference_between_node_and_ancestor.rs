/**
 * [1026] Maximum Difference Between Node and Ancestor
 *
 * Given the root of a binary tree, find the maximum value v for which there exist different nodes a and b where v = |a.val - b.val| and a is an ancestor of b.
 * A node a is an ancestor of b if either: any child of a is equal to b or any child of a is an ancestor of b.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/09/tmp-tree.jpg" style="width: 400px; height: 390px;" />
 * Input: root = [8,3,10,1,6,null,14,null,null,4,7,13]
 * Output: 7
 * Explanation: We have various ancestor-node differences, some of which are given below :
 * |8 - 3| = 5
 * |3 - 7| = 4
 * |8 - 1| = 7
 * |10 - 13| = 3
 * Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/09/tmp-tree-1.jpg" style="width: 250px; height: 349px;" />
 * Input: root = [1,null,2,null,0,3]
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [2, 5000].
 *     0 <= Node.val <= 10^5
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
	pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::solve(&root.as_ref().unwrap().borrow()).2
	}
	pub fn solve(root: &TreeNode) -> (i32, i32, i32) {
		let (mut min, mut max, mut diff) = (root.val, root.val, 0);
		for no in [&root.left, &root.right] {
			if let Some(no) = no.as_ref() {
				let v = Self::solve(&no.borrow());
				min = min.min(v.0);
				max = max.max(v.1);
				diff = diff.max(v.2);
			}
		}
		diff = diff.max((root.val - min).abs()).max((root.val - max).abs());
		(min, max, diff)
	}
}

// submission codes end
