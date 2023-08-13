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
		mut head: Option<Box<ListNode>>,
		left: i32,
		right: i32,
	) -> Option<Box<ListNode>> {
		let mut cur = &mut head;
		for _ in 1..left {
			cur = &mut cur.as_mut().unwrap().next;
		}
		let mut rev = cur.take();
		let mut cur_rev = &mut rev;
		for _ in left..=right {
			cur_rev = &mut cur_rev.as_mut().unwrap().next;
		}
		*cur = cur_rev.take();
		while let Some(mut no) = rev {
			rev = std::mem::replace(&mut no.next, cur.take());
			*cur = Some(no);
		}
		head
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
