/**
 * [872] Leaf-Similar Trees
 *
 * Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/16/tree.png" style="width: 400px; height: 336px;" />
 * For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).
 * Two binary trees are considered leaf-similar if their leaf value sequence is the same.
 * Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/leaf-similar-1.jpg" style="width: 600px; height: 237px;" />
 * Input: root1 = [3,5,1,6,2,9,8,null,null,7,4], root2 = [3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]
 * Output: true
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/leaf-similar-2.jpg" style="width: 300px; height: 110px;" />
 * Input: root1 = [1,2,3], root2 = [1,3,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     The number of nodes in each tree will be in the range [1, 200].
 *     Both of the given trees will have values in the range [0, 200].
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
	pub fn leaf_similar(
		root1: Option<Rc<RefCell<TreeNode>>>,
		root2: Option<Rc<RefCell<TreeNode>>>,
	) -> bool {
		let (mut ans1, mut ans2) = (vec![], vec![]);
		Self::inner(&root1.unwrap().borrow(), &mut ans1);
		Self::inner(&root2.unwrap().borrow(), &mut ans2);
		ans1 == ans2
	}
	fn inner(root: &TreeNode, ans: &mut Vec<i32>) {
		match (&root.left, &root.right) {
			(Some(l), Some(r)) => {
				Self::inner(&l.borrow(), ans);
				Self::inner(&r.borrow(), ans);
			}
			(Some(no), _) | (_, Some(no)) => Self::inner(&no.borrow(), ans),
			_ => ans.push(root.val),
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_872() {}
}
