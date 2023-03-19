/**
 * [101] Symmetric Tree
 *
 * Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree1.jpg" style="width: 354px; height: 291px;" />
 * Input: root = [1,2,2,3,4,4,3]
 * Output: true
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree2.jpg" style="width: 308px; height: 258px;" />
 * Input: root = [1,2,2,null,3,null,3]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 1000].
 *     -100 <= Node.val <= 100
 *
 *  
 * Follow up: Could you solve it both recursively and iteratively?
 */
pub struct Solution {}
use super::util::tree::TreeNode;

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		match root {
			None => true,
			Some(root) => {
				let root = root.borrow();
				Self::is_sym(&root.left, &root.right)
			}
		}
	}

	pub fn is_sym(l: &Option<Rc<RefCell<TreeNode>>>, r: &Option<Rc<RefCell<TreeNode>>>) -> bool {
		match (l, r) {
			(None, None) => true,
			(Some(l), Some(r)) => {
				let (l, r) = (l.borrow(), r.borrow());
				(l.val == r.val)
					&& Self::is_sym(&l.left, &r.right)
					&& Self::is_sym(&l.right, &r.left)
			}
			_ => false,
		}
	}
}
