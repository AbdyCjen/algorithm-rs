/**
 * [652] Find Duplicate Subtrees
 *
 * Given the root of a binary tree, return all duplicate subtrees.
 * For each kind of duplicate subtrees, you only need to return the root node of any one of them.
 * Two trees are duplicate if they have the same structure with the same node values.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e1.jpg" style="width: 450px; height: 354px;" />
 * Input: root = [1,2,3,4,null,2,4,null,null,4]
 * Output: [[2,4],[4]]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e2.jpg" style="width: 321px; height: 201px;" />
 * Input: root = [2,1,1]
 * Output: [[1]]
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e33.jpg" style="width: 450px; height: 303px;" />
 * Input: root = [2,2,2,3,null,3,null]
 * Output: [[2,3],[3]]
 *
 *  
 * Constraints:
 *
 *     The number of the nodes in the tree will be in the range [1, 5000]
 *     -200 <= Node.val <= 200
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
use std::{borrow::Cow, cell::RefCell, rc::Rc};
impl Solution {
	pub fn find_duplicate_subtrees(
		root: Option<Rc<RefCell<TreeNode>>>,
	) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
		let mut map = std::collections::HashMap::new();
		Self::solve(&root, &mut map);
		map.into_values()
			.filter_map(|mut v| if v.len() > 1 { v.pop() } else { None })
			.collect()
	}

	fn solve(
		root: &Option<Rc<RefCell<TreeNode>>>,
		map: &mut std::collections::HashMap<Cow<'static, str>, Vec<Option<Rc<RefCell<TreeNode>>>>>,
	) -> Cow<'static, str> {
		let r = match root {
			Some(r) => r.borrow(),
			_ => return Cow::Borrowed("null"),
		};
		let s = Cow::Owned(format!(
			"{},{},{}",
			r.val,
			Self::solve(&r.left, map),
			Self::solve(&r.right, map)
		));

		map.entry(Clone::clone(&s)).or_default().push(root.clone());
		s
	}
}

// submission codes end
