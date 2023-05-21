/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [2,1,4,3]
 *
 * <strong class="example">Example 2:
 *
 * Input: head = []
 * Output: []
 *
 * <strong class="example">Example 3:
 *
 * Input: head = [1]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [0, 100].
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
	pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut cur = head.as_mut();
		while let Some((Some(mut ne), no)) = cur.map(|no| (no.next.take(), no)) {
			let tail = ne.next.take();
			ne = std::mem::replace(no, ne);
			let ne = no.next.insert(ne);
			ne.next = tail;
			cur = ne.next.as_mut();
		}
		head
	}
}

// submission codes end
