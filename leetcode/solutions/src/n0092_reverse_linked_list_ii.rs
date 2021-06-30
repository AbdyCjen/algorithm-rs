/**
 * [92] Reverse Linked List II
 *
 * Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev2ex2.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], left = 2, right = 4
 * Output: [1,4,3,2,5]
 *
 * Example 2:
 *
 * Input: head = [5], left = 1, right = 1
 * Output: [5]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is n.
 *     1 <= n <= 500
 *     -500 <= Node.val <= 500
 *     1 <= left <= right <= n
 *
 *  
 * Follow up: Could you do it in one pass?
 */
pub struct Solution {}
use super::util::linked_list::ListNode;

// submission codes start here

#[allow(dead_code)]
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
	pub fn reverse_between(
		head: Option<Box<ListNode>>,
		left: i32,
		right: i32,
	) -> Option<Box<ListNode>> {
		let mut dummy = ListNode::new(0);
		dummy.next = head;
		let mut p = &mut dummy;
		for _ in 0..left - 1 {
			p = p.next.as_deref_mut().unwrap();
		}

		let mut rev_head = None;
		let mut to_rev = p.next.take();
		for _ in left..=right {
			let mut to_rev_head = to_rev.unwrap();
			to_rev = to_rev_head.next.take();
			to_rev_head.next = rev_head;
			rev_head = Some(to_rev_head);
		}
		//append to_rev to rev_head
		let mut cur = &mut rev_head;
		while let Some(cur_ptr) = cur {
			cur = &mut cur_ptr.next;
		}
		*cur = to_rev;
		p.next = rev_head;
		dummy.next.take()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::linked_list::to_list, *};

	#[test]
	fn test_92() {
		assert_eq!(
			Solution::reverse_between(linked![1, 2, 3, 4, 5], 2, 4),
			linked![1, 4, 3, 2, 5]
		);
		assert_eq!(Solution::reverse_between(linked![5], 1, 1), linked![5]);
	}
}
