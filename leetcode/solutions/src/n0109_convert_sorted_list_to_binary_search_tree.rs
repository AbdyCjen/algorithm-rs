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

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
		let mut len = 0;
		let mut cur = &mut head;
		while let Some(no) = cur {
			len += 1;
			cur = &mut no.next;
		}
		*cur = Some(Box::new(ListNode::new(0)));

		head.as_mut().and_then(|h| Self::solve(h, len))
	}
	fn solve(head: &mut Box<ListNode>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
		if len == 0 {
			return None;
		}
		let ans = Some(Rc::new(RefCell::new(TreeNode {
			left: Self::solve(head, len / 2),
			val: head.val,
			right: Self::solve(head.next.as_mut()?, (len - 1) / 2),
		})));
		*head = head.next.take()?;
		ans
	}
}
