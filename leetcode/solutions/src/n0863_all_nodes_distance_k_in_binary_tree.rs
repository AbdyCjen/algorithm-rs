/**
 * [893] All Nodes Distance K in Binary Tree
 *
 * We are given a binary tree (with root node root), a target node, and an integer value K.
 *
 * Return a list of the values of all nodes that have a distance K from the target node.  The answer can be returned in any order.
 *
 *  
 *
 * <ol>
 * </ol>
 *
 * <div>
 * Example 1:
 *
 *
 * Input: root = <span id="example-input-1-1">[3,5,1,6,2,0,8,null,null,7,4]</span>, target = <span id="example-input-1-2">5</span>, K = <span id="example-input-1-3">2</span>
 *
 * Output: <span id="example-output-1">[7,4,1]</span>
 *
 * Explanation:
 * The nodes that are a distance 2 from the target node (with value 5)
 * have values 7, 4, and 1.
 *
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png" style="width: 280px; height: 240px;" />
 *
 * Note that the inputs "root" and "target" are actually TreeNodes.
 * The descriptions of the inputs above are just serializations of these objects.
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 *  The given tree is non-empty.
 *  Each node in the tree has unique values 0 <= node.val <= 500.
 *  The target node is a node in the tree.
 *  0 <= K <= 1000.
 * </ol>
 * </div>
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
	fn distance(root: &TreeNode, k: u32, ans: &mut Vec<i32>) {
		match k.checked_sub(1) {
			None => ans.push(root.val),
			Some(k) => {
				for &ch in [root.left.as_ref(), root.right.as_ref()].iter().flatten() {
					Self::distance(&ch.borrow(), k, ans);
				}
			}
		}
	}
	pub fn distance_k(
		root: Option<Rc<RefCell<TreeNode>>>,
		p: Option<Rc<RefCell<TreeNode>>>,
		k: i32,
	) -> Vec<i32> {
		let mut ans = Vec::new();
		Self::solve(&root, p.as_ref().unwrap().borrow().val, k as u32, &mut ans);
		ans
	}
	fn solve(
		root: &Option<Rc<RefCell<TreeNode>>>,
		tar: i32,
		k: u32,
		ans: &mut Vec<i32>,
	) -> Option<u32> {
		let root = root.as_ref()?.borrow();
		if root.val == tar {
			Self::distance(&root, k, ans);
			Some(k - 1)
		} else {
			for (l, r) in [(&root.left, &root.right), (&root.right, &root.left)] {
				if let Some(k) = Self::solve(l, tar, k, ans) {
					match k.checked_sub(1) {
						None => ans.push(root.val),
						Some(k) => {
							if let Some(r) = r {
								Self::distance(&r.borrow(), k, ans)
							}
						}
					};
					return k.checked_sub(1);
				}
			}
			None
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	#[test]
	fn test_893() {}
}
