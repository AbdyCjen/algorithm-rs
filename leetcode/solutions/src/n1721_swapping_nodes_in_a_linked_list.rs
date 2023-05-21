/**
 * [1721] Swapping Nodes in a Linked List
 *
 * You are given the head of a linked list, and an integer k.
 * Return the head of the linked list after swapping the values of the k^th node from the beginning and the k^th node from the end (the list is 1-indexed).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/linked1.jpg" style="width: 400px; height: 112px;" />
 * Input: head = [1,2,3,4,5], k = 2
 * Output: [1,4,3,2,5]
 *
 * <strong class="example">Example 2:
 *
 * Input: head = [7,9,6,6,7,8,3,0,9,5], k = 5
 * Output: [7,9,6,6,8,7,3,0,9,5]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is n.
 *     1 <= k <= n <= 10^5
 *     0 <= Node.val <= 100
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
	pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
		let len = std::iter::successors(head.as_ref(), |no| no.next.as_ref()).count() as i32;
		let p = len - k + 1;
		let (k, p) = (k.min(p), k.max(p));
		if k == p {
			return head;
		}

		let mut cur = head.as_mut().unwrap();
		for _ in 1..k {
			cur = cur.next.as_mut().unwrap();
		}
		let mut tail = cur.next.take();
		let mut next = tail.as_mut().unwrap();
		for _ in k + 1..p {
			next = next.next.as_mut().unwrap();
		}
		std::mem::swap(&mut cur.val, &mut next.val);
		cur.next = tail;

		head
	}
}

// submission codes end
