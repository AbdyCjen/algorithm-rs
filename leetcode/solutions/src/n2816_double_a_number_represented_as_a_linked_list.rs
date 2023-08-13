/**
 * [2816] Double a Number Represented as a Linked List
 *
 * You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.
 * Return the head of the linked list after doubling it.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/05/28/example.png" style="width: 401px; height: 81px;" />
 * Input: head = [1,8,9]
 * Output: [3,7,8]
 * Explanation: The figure above corresponds to the given linked list which represents the number 189. Hence, the returned linked list represents the number 189 * 2 = 378.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/05/28/example2.png" style="width: 401px; height: 81px;" />
 * Input: head = [9,9,9]
 * Output: [1,9,9,8]
 * Explanation: The figure above corresponds to the given linked list which represents the number 999. Hence, the returned linked list reprersents the number 999 * 2 = 1998.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [1, 10^4]
 *     <font face="monospace">0 <= Node.val <= 9</font>
 *     The input is generated such that the list represents a number that does not have leading zeros, except the number 0 itself.
 *
 */
pub struct Solution {}
use super::util::linked_list::ListNode;

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
impl Solution {
	pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut head = Self::rev(head);
		let mut flow = 0;
		let mut cur = &mut head;
		while let Some(no) = cur {
			no.val = no.val * 2 + flow;
			flow = no.val / 10;
			no.val %= 10;
			cur = &mut no.next;
		}
		let head = Self::rev(head);
		if flow > 0 {
			Some(Box::new(ListNode { val: 1, next: head }))
		} else {
			head
		}
	}
	fn rev(mut lis: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut h = None;
		while let Some(mut l) = lis {
			lis = std::mem::replace(&mut l.next, h);
			h = Some(l);
		}
		h
	}
}

// submission codes end
