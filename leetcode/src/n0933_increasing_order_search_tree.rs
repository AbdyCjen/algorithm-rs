/**
 * [933] Increasing Order Search Tree
 *
 * Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/ex1.jpg" style="width: 600px; height: 350px;" />
 *
 * Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
 * Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
 *
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/ex2.jpg" style="width: 300px; height: 114px;" />
 *
 * Input: root = [5,1,7]
 * Output: [1,null,5,null,7]
 *
 *
 *  
 * Constraints:
 *
 *
 * 	The number of nodes in the given tree will be in the range [1, 100].
 * 	0 <= Node.val <= 1000
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
//好艹啊,
//如果按中序遍历来，保存一个链状二叉树,　并且持有这个链尾部&mut,
//但是refcell下必须要获取链上所有节点的borrow结构才行, 看下能不能反过来
use std::{cell::RefCell, rc::Rc};
#[allow(dead_code)]
impl Solution {
	pub fn increasing_bst(
		mut root: Option<Rc<RefCell<TreeNode>>>,
	) -> Option<Rc<RefCell<TreeNode>>> {
		let mut st = Vec::new();
		while let Some(o) = root.take() {
			root = o.borrow_mut().right.take();
			st.push(o);
		}
		while let Some(o) = st.pop() {
			let l = o.borrow_mut().left.take();
			o.borrow_mut().right = root;
			root = Some(o);
			if let Some(l) = l {
				let mut r = l.borrow_mut().right.take();
				st.push(l);
				while let Some(o) = r.take() {
					r = o.borrow_mut().right.take();
					st.push(o);
				}
			}
		}
		root
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_933() {
		assert_eq!(
			Solution::increasing_bst(tree![5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9]),
			tree![1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9]
		);
		assert_eq!(
			Solution::increasing_bst(tree![5, 1, 7]),
			tree![1, null, 5, null, 7]
		);
	}
}
