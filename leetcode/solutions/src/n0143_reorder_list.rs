/**
 * [143] Reorder List
 *
 * You are given the head of a singly linked-list. The list can be represented as:
 *
 * L0 &rarr; L1 &rarr; &hellip; &rarr; Ln - 1 &rarr; Ln
 *
 * Reorder the list to be on the following form:
 *
 * L0 &rarr; Ln &rarr; L1 &rarr; Ln - 1 &rarr; L2 &rarr; Ln - 2 &rarr; &hellip;
 *
 * You may not modify the values in the list's nodes. Only nodes themselves may be changed.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/04/reorder1linked-list.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [1,4,2,3]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/reorder2-linked-list.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [1,5,2,4,3]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [1, 5 * 10^4].
 *     1 <= Node.val <= 1000
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
	pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
		let len = std::iter::successors(head.as_ref(), |no| no.next.as_ref()).count();
		let mut cur = &mut *head;
		for _ in 0..(len + 1) / 2 {
			cur = &mut cur.as_mut().unwrap().next;
		}

		let mut tail = cur.take();
		let mut new = None;
		while let Some(mut ta) = tail {
			tail = std::mem::replace(&mut ta.next, new);
			new = Some(ta);
		}
		let mut cur = head;
		while let (Some(ref mut no), Some(mut ta)) = (cur, new) {
			new = std::mem::replace(&mut ta.next, no.next.take());
			cur = &mut no.next.insert(ta).next;
		}
	}
}

// submission codes end
