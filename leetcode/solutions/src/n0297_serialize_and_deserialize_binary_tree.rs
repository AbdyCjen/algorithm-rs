/**
 * [297] Serialize and Deserialize Binary Tree
 *
 * Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
 * Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
 * Clarification: The input/output format is the same as <a href="https://support.leetcode.com/hc/en-us/articles/360011883654-What-does-1-null-2-3-mean-in-binary-tree-representation-" target="_blank">how LeetCode serializes a binary tree</a>. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/serdeser.jpg" style="width: 442px; height: 324px;" />
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 *
 * <strong class="example">Example 2:
 *
 * Input: root = []
 * Output: []
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [0, 10^4].
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
use std::{cell::RefCell, fmt::Write as _, rc::Rc};
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
	fn new() -> Self { Self {} }

	fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
		fn fmt(s: &mut String, root: &Option<Rc<RefCell<TreeNode>>>) {
			match root.as_deref().map(RefCell::borrow) {
				Some(no) => {
					let _ = write!(s, "{},", no.val);
					fmt(s, &no.left);
					fmt(s, &no.right);
				}
				None => s.push(','),
			}
		}
		let mut s = String::new();
		fmt(&mut s, &root);
		s
	}

	fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
		fn deser<'a>(it: &mut impl Iterator<Item = &'a str>) -> Option<Rc<RefCell<TreeNode>>> {
			let tr = TreeNode {
				val: it.next()?.parse().ok()?,
				left: deser(it),
				right: deser(it),
			};
			Rc::new(RefCell::new(tr)).into()
		}
		deser(&mut data.split(','))
	}
}

// submission codes end
