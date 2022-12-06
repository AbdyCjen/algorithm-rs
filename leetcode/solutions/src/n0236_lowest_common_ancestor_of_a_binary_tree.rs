/**
 * [236] Lowest Common Ancestor of a Binary Tree
 *
 * Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
 * According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).&rdquo;
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
 * Output: 3
 * Explanation: The LCA of nodes 5 and 1 is 3.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
 * Output: 5
 * Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1,2], p = 1, q = 2
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [2, 10^5].
 *     -10^9 <= Node.val <= 10^9
 *     All Node.val are unique.
 *     p != q
 *     p and q will exist in the tree.
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
	pub fn lowest_common_ancestor(
		root: Option<Rc<RefCell<TreeNode>>>,
		p: Option<Rc<RefCell<TreeNode>>>,
		q: Option<Rc<RefCell<TreeNode>>>,
	) -> Option<Rc<RefCell<TreeNode>>> {
		let p = p.unwrap();
		let q = q.unwrap();
		let p_b = p.borrow_mut();
		if p == q {
			drop(root);
		}
		drop(p_b);
		None
	}
	pub fn lowest_common_ancestor_1(
		root: Option<Rc<RefCell<TreeNode>>>,
		p: Option<Rc<RefCell<TreeNode>>>,
		q: Option<Rc<RefCell<TreeNode>>>,
	) -> Option<Rc<RefCell<TreeNode>>> {
		Self::lca(&root, p.unwrap().borrow().val, q.unwrap().borrow().val).err()
	}

	fn lca(
		root_opt: &Option<Rc<RefCell<TreeNode>>>,
		p: i32,
		q: i32,
	) -> Result<Option<()>, Rc<RefCell<TreeNode>>> {
		match root_opt {
			Some(root) => {
				let r = root.borrow();
				//dbg!((r.val, p, q));
				if r.val == p || r.val == q {
					if let Ok(Some(_)) = Self::lca(&r.left, q, p) {
						Err(root.clone())
					} else if let Ok(Some(_)) = Self::lca(&r.right, q, p) {
						Err(root.clone())
					} else {
						Ok(Some(()))
					}
				} else if Self::lca(&r.left, p, q)?.is_some() {
					if Self::lca(&r.right, q, p)?.is_some() {
						Err(root.clone())
					} else {
						Ok(Some(()))
					}
				} else if Self::lca(&r.right, p, q)?.is_some() {
					Ok(Some(()))
				} else {
					Ok(None)
				}
			}
			_ => Ok(None),
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_236() {
		assert_eq!(
			Solution::lowest_common_ancestor(tree![2, null, 1], tree![1], tree![2])
				.unwrap()
				.borrow()
				.val,
			2
		);
		assert_eq!(
			Solution::lowest_common_ancestor(tree![1, 2], tree![2], tree![1])
				.unwrap()
				.borrow()
				.val,
			1
		);
		assert_eq!(
			Solution::lowest_common_ancestor(
				tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
				tree![5],
				tree![4]
			)
			.unwrap()
			.borrow()
			.val,
			5
		);
		assert_eq!(
			Solution::lowest_common_ancestor(
				tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4],
				tree![5],
				tree![1]
			)
			.unwrap()
			.borrow()
			.val,
			3
		);
		assert_eq!(
			Solution::lowest_common_ancestor(tree![1, 2], tree![1], tree![2])
				.unwrap()
				.borrow()
				.val,
			1
		);
	}
}
