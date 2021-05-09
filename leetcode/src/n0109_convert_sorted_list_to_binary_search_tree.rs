/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given the head of a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
 * For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/linked.jpg" style="width: 600px; height: 466px;" />
 * Input: head = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
 *
 * Example 2:
 *
 * Input: head = []
 * Output: []
 *
 * Example 3:
 *
 * Input: head = [0]
 * Output: [0]
 *
 * Example 4:
 *
 * Input: head = [1,3]
 * Output: [3,1]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in head is in the range [0, 2 * 10^4].
 *     -10^5 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use super::util::{linked_list::ListNode, tree::TreeNode};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
	pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
		fn sorted_list_to_bst_inner(
			head: Option<Box<ListNode>>,
			len: usize,
		) -> (Option<Box<ListNode>>, Option<Rc<RefCell<TreeNode>>>) {
			if len == 0 {
				return (head, None);
			}
			let r = (len - 1) / 2;
			let l = len - r - 1;
			let (head, lt) = sorted_list_to_bst_inner(head, l);
			let mut tr = TreeNode::new(head.as_ref().unwrap().val);
			let (head, rt) = sorted_list_to_bst_inner(head.unwrap().next, r);
			tr.left = lt;
			tr.right = rt;
			(head, Some(Rc::new(RefCell::new(tr))))
		}
		let mut len = 0;
		let mut cur = &head;
		while let Some(cur_node) = cur {
			len += 1;
			cur = &cur_node.next;
		}

		sorted_list_to_bst_inner(head, len).1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{
		super::util::{linked_list::to_list, tree::to_tree},
		*,
	};

	#[test]
	fn test_109() {
		assert_eq!(
			Solution::sorted_list_to_bst(linked![-10, -3, 0, 5, 9]),
			tree![0, -3, 9, -10, null, 5]
		);
		assert_eq!(
			Solution::sorted_list_to_bst(linked![-10, -3, 0, 5, 9]),
			tree![0, -3, 9, -10, null, 5]
		);
	}
}
