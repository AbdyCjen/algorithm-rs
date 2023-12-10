/**
 * [606] Construct String from Binary Tree
 *
 * Given the root of a binary tree, construct a string consisting of parenthesis and integers from a binary tree with the preorder traversal way, and return it.
 * Omit all the empty parenthesis pairs that do not affect the one-to-one mapping relationship between the string and the original binary tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/cons1-tree.jpg" style="width: 292px; height: 301px;" />
 * Input: root = [1,2,3,4]
 * Output: "1(2(4))(3)"
 * Explanation: Originally, it needs to be "1(2(4)())(3()())", but you need to omit all the unnecessary empty parenthesis pairs. And it will be "1(2(4))(3)"
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/cons2-tree.jpg" style="width: 207px; height: 293px;" />
 * Input: root = [1,2,3,null,4]
 * Output: "1(2()(4))(3)"
 * Explanation: Almost the same as the first example, except we cannot omit the first parenthesis pair to break the one-to-one mapping relationship between the input and the output.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 10^4].
 *     -1000 <= Node.val <= 1000
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
	pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
		let mut ans = "".to_owned();
		if let Some(root) = root {
			Self::solve(&root.borrow(), &mut ans);
		}
		ans
	}

	fn solve(root: &TreeNode, s: &mut String) {
		use std::fmt::Write;
		write!(s, "{}", root.val).unwrap();
		match (&root.left, &root.right) {
			(None, None) => {}
			(l, r) => {
				s.push('(');
				if let Some(l) = l {
					Self::solve(&l.borrow(), s);
				}
				s.push(')');
				if let Some(r) = r {
					s.push('(');
					Self::solve(&r.borrow(), s);
					s.push(')');
				}
			}
		}
	}
}

// submission codes end
