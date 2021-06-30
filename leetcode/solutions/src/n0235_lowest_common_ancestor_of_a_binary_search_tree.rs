/**
 * [235] Lowest Common Ancestor of a Binary Search Tree
 *
 * Given a binary search tree (BST), find the lowest common ancestor (LCA) of two given nodes in the BST.
 * According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).&rdquo;
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png" style="width: 200px; height: 190px;" />
 * Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
 * Output: 6
 * Explanation: The LCA of nodes 2 and 8 is 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png" style="width: 200px; height: 190px;" />
 * Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
 * Output: 2
 * Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.
 *
 * Example 3:
 *
 * Input: root = [2,1], p = 2, q = 1
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [2, 10^5].
 *     -10^9 <= Node.val <= 10^9
 *     All Node.val are unique.
 *     p != q
 *     p and q will exist in the BST.
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
#[allow(dead_code)]
impl Solution {
	pub fn lowest_common_ancestor(
		root: Option<Rc<RefCell<TreeNode>>>,
		p: Option<Rc<RefCell<TreeNode>>>,
		q: Option<Rc<RefCell<TreeNode>>>,
	) -> Option<Rc<RefCell<TreeNode>>> {
		fn lca(root_raw: Rc<RefCell<TreeNode>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
			{
				let mut root = root_raw.borrow_mut();
				if p < root.val && q < root.val {
					return lca(root.left.take()?, p, q);
				} else if p > root.val && q > root.val {
					return lca(root.right.take()?, p, q);
				}
			}
			Some(root_raw)
		}
		let p = p.unwrap().borrow().val;
		let q = q.unwrap().borrow().val;
		lca(root.unwrap(), p, q)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::tree::*;
	#[test]
	fn test_235() {
		assert_eq!(
			Solution::lowest_common_ancestor(
				tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5],
				tree![2],
				tree![8]
			),
			tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]
		);
		assert_eq!(
			Solution::lowest_common_ancestor(tree![2, 1], tree![2], tree![1]),
			tree![2, 1]
		);
	}
}
