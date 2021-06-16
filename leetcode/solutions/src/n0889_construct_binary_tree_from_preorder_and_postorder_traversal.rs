/**
 * [889] Construct Binary Tree from Preorder and Postorder Traversal
 *
 * Return any binary tree that matches the given preorder and postorder traversals.
 *
 * Values in the traversals pre and post are distinct positive integers.
 *
 *  
 *
 * <div>
 * Example 1:
 *
 *
 * Input: pre = <span id="example-input-1-1">[1,2,4,5,3,6,7]</span>, post = <span id="example-input-1-2">[4,5,2,6,7,3,1]</span>
 * Output: <span id="example-output-1">[1,2,3,4,5,6,7]</span>
 *
 *
 *  
 *
 * <span>Note:</span>
 *
 *
 *     1 <= pre.length == post.length <= 30
 *     pre[] and post[] are both permutations of 1, 2, ..., pre.length.
 *     It is guaranteed an answer exists. If there exists multiple answers, you can return any of them.
 *
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
#[allow(dead_code)]
impl Solution {
	pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		use std::iter::Peekable;
		fn construct_inner<I: Iterator<Item = i32>>(
			pre: &mut Peekable<I>,
			post: &mut Peekable<I>,
		) -> Option<Rc<RefCell<TreeNode>>> {
			let val = pre.next()?;
			let mut get_one = || {
				if Some(&val) != post.peek() {
					construct_inner(pre, post)
				} else {
					None
				}
			};
			let left = get_one();
			let right = get_one();
			post.next();

			Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
		}

		construct_inner(
			&mut pre.into_iter().peekable(),
			&mut post.into_iter().peekable(),
		)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_889() {
		assert_eq!(
			Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]),
			tree![1, 2, 3, 4, 5, 6, 7]
		);
	}
}
