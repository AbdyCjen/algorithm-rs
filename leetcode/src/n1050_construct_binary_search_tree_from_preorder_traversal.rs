/**
 * [1050] Construct Binary Search Tree from Preorder Traversal
 *
 * Return the root node of a binary search tree that matches the given preorder traversal.
 * (Recall that a binary search tree is a binary tree where for every <font face="monospace">node</font>, any descendant of node.left has a value < node.val, and any descendant of node.right has a value > node.val.  Also recall that a preorder traversal displays the value of the node first, then traverses node.left, then traverses node.right.)
 * It's guaranteed that for the given test cases there is always possible to find a binary search tree with the given requirements.
 * Example 1:
 *
 * Input: <span id="example-input-1-1">[8,5,1,7,10,12]</span>
 * Output: <span id="example-output-1">[8,5,10,1,7,null,12]
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/03/06/1266.png" style="height: 200px; width: 306px;" /></span>
 *
 *  
 * Constraints:
 *
 * 1 <= preorder.length <= 100
 * 1 <= preorder[i] <= 10^8
 * The values of preorder are distinct.
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
	fn tree_new(val: i32) -> TreeNode {
		TreeNode {
			val,
			left: None,
			right: None,
		}
	}
	fn bst_from_slc(slc: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
		let mut root = Self::tree_new(*slc.get(0)?);
		let slc = &slc[1..];
		let mut i = 0;
		// shit, use iterator
		for v in slc {
			if *v < root.val {
				i += 1;
			} else {
				break;
			}
		}
		root.left = Self::bst_from_slc(&slc[..i]);
		root.right = Self::bst_from_slc(&slc[i..]);
		Some(Rc::new(RefCell::new(root)))
	}
	pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		Self::bst_from_slc(&preorder)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_1050() {
		assert_eq!(
			Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
			tree![8, 5, 10, 1, 7, null, 12]
		);
	}
}
