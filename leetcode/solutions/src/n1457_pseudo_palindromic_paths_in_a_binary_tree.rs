/**
 * [1457] Pseudo-Palindromic Paths in a Binary Tree
 *
 * Given a binary tree where node values are digits from 1 to 9. A path in the binary tree is said to be pseudo-palindromic if at least one permutation of the node values in the path is a palindrome.
 * Return the number of pseudo-palindromic paths going from the root node to leaf nodes.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/06/palindromic_paths_1.png" style="width: 300px; height: 201px;" />
 *
 * Input: root = [2,3,1,3,1,null,1]
 * Output: 2
 * Explanation: The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the red path [2,3,3], the green path [2,1,1], and the path [2,3,1]. Among these paths only red path and green path are pseudo-palindromic paths since the red path [2,3,3] can be rearranged in [3,2,3] (palindrome) and the green path [2,1,1] can be rearranged in [1,2,1] (palindrome).
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/palindromic_paths_2.png" style="width: 300px; height: 314px;" />
 *
 * Input: root = [2,1,1,1,3,null,null,null,null,null,1]
 * Output: 1
 * Explanation: The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the green path [2,1,1], the path [2,1,3,1], and the path [2,1]. Among these paths only the green path is pseudo-palindromic since [2,1,1] can be rearranged in [1,2,1] (palindrome).
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [9]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 10^5].
 *     1 <= Node.val <= 9
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
	pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut st = vec![(root.unwrap(), 0_i32)];
		let mut ans = 0;
		while let Some((no, vis)) = st.pop() {
			let no = no.borrow();
			let vis = vis ^ (1 << no.val);
			match (no.left.clone(), no.right.clone()) {
				(None, None) => ans += (vis.count_ones() <= 1) as i32,
				(Some(l), Some(r)) => st.extend([(l, vis), (r, vis)]),
				(Some(ch), _) | (_, Some(ch)) => st.push((ch, vis)),
			}
		}
		ans
	}
	pub fn pseudo_palindromic_paths1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		Self::solve(&*root.as_ref().unwrap().borrow(), 0)
	}

	fn solve(root: &TreeNode, vis: i32) -> i32 {
		let vis = vis ^ (1 << root.val);
		match (&root.left, &root.right) {
			(None, None) => (vis.count_ones() <= 1) as i32,
			(Some(l), Some(r)) => Self::solve(&*l.borrow(), vis) + Self::solve(&*r.borrow(), vis),
			(Some(ch), _) | (_, Some(ch)) => Self::solve(&*ch.borrow(), vis),
		}
	}
}

// submission codes end
