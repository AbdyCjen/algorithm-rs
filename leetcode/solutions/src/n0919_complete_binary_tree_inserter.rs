/**
 * [919] Complete Binary Tree Inserter
 *
 * A complete binary tree is a binary tree in which every level, except possibly the last, is completely filled, and all nodes are as far left as possible.
 * Design an algorithm to insert a new node to a complete binary tree keeping it complete after the insertion.
 * Implement the CBTInserter class:
 *
 *     CBTInserter(TreeNode root) Initializes the data structure with the root of the complete binary tree.
 *     int insert(int v) Inserts a TreeNode into the tree with value Node.val == val so that the tree remains complete, and returns the value of the parent of the inserted TreeNode.
 *     TreeNode get_root() Returns the root node of the tree.
 *
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/lc-treeinsert.jpg" style="width: 500px; height: 143px;" />
 * Input
 * ["CBTInserter", "insert", "insert", "get_root"]
 * [[[1, 2]], [3], [4], []]
 * Output
 * [null, 1, 2, [1, 2, 3, 4]]
 * Explanation
 * CBTInserter cBTInserter = new CBTInserter([1, 2]);
 * cBTInserter.insert(3);  // return 1
 * cBTInserter.insert(4);  // return 2
 * cBTInserter.get_root(); // return [1, 2, 3, 4]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree will be in the range [1, 1000].
 *     0 <= Node.val <= 5000
 *     root is a complete binary tree.
 *     0 <= val <= 5000
 *     At most 10^4 calls will be made to insert and get_root.
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
struct CBTInserter(Vec<Rc<RefCell<TreeNode>>>);

impl CBTInserter {
	fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
		let mut v = vec![root.unwrap()];
		let mut i = 0;
		while i < v.len() {
			let cur = v[i].clone();
			match (cur.borrow().left.clone(), cur.borrow().right.clone()) {
				(Some(l), r) => {
					v.push(l);
					match r {
						Some(r) => v.push(r),
						_ => break,
					}
				}
				_ => break,
			}
			i += 1;
		}
		Self(v)
	}

	fn insert(&mut self, val: i32) -> i32 {
		let len = self.0.len();
		let par = (len - 1) / 2;
		let no = Rc::new(RefCell::new(TreeNode::new(val)));
		if len % 2 == 1 {
			self.0[par].borrow_mut().left = Some(no.clone());
		} else {
			self.0[par].borrow_mut().right = Some(no.clone());
		}
		self.0.push(no);
		self.0[par].borrow().val
	}

	fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> { Some(self.0[0].clone()) }
}

// submission codes end
