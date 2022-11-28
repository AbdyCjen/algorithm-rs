/**
 * [2487] Remove Nodes From Linked List
 *
 * You are given the head of a linked list.
 * Remove every node which has a node with a strictly greater value anywhere to the right side of it.
 * Return the head of the modified linked list.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/02/drawio.png" style="width: 631px; height: 51px;" />
 * Input: head = [5,2,13,3,8]
 * Output: [13,8]
 * Explanation: The nodes that should be removed are 5, 2 and 3.
 * - Node 13 is to the right of node 5.
 * - Node 13 is to the right of node 2.
 * - Node 8 is to the right of node 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: head = [1,1,1,1]
 * Output: [1,1,1,1]
 * Explanation: Every node has value 1, so no nodes are removed.
 *
 *  
 * Constraints:
 *
 *     The number of the nodes in the given list is in the range [1, 10^5].
 *     1 <= Node.val <= 10^5
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
	pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut tail: Option<Box<ListNode>> = None;
		while let Some(mut h) = head {
			head = h.next.take();
			while let Some(t) = tail.as_mut() {
				if t.val >= h.val {
					break;
				} else {
					tail = t.next.take();
				}
			}
			h.next = tail;
			tail = Some(h);
		}
		Self::rev(tail)
	}

	fn rev(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut list = None;
		while let Some(mut h) = head {
			head = h.next.take();
			h.next = list;
			list = Some(h);
		}
		list
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::linked_list::to_list, *};

	#[test]
	fn test_2487() {
		assert_eq!(
			Solution::remove_nodes(linked![5, 2, 13, 3, 8]),
			linked![13, 8]
		);
		assert_eq!(Solution::remove_nodes(None), None);
	}
}
