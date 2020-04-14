/**
 * [908] Middle of the Linked List
 *
 * Given a non-empty, singly linked list with head node head, return a middle node of linked list.
 *
 * If there are two middle nodes, return the second middle node.
 *
 *  
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[1,2,3,4,5]</span>
 * Output: Node 3 from this list (Serialization: <span id="example-output-1">[3,4,5]</span>)
 * The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
 * Note that we returned a ListNode object ans, such that:
 * ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,2,3,4,5,6]</span>
 * Output: Node 4 from this list (Serialization: <span id="example-output-2">[4,5,6]</span>)
 * Since the list has two middle nodes with values 3 and 4, we return the second one.
 *
 *
 *  
 *
 * Note:
 *
 *
 * 	The number of nodes in the given list will be between 1 and 100.
 *
 * </div>
 * </div>
 *
 */
#[allow(dead_code)]
pub struct Solution {}
use super::util::linked_list::{to_list, ListNode};

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
	pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut cnt_bf_mid = 0;
		let mut o: &Option<Box<ListNode>> = &head;
		while let Some(oo) = o {
			o = &oo.next;
			cnt_bf_mid += 1;
		}
		cnt_bf_mid /= 2;
		for _ in 0..cnt_bf_mid {
			head = head?.next.take();
		}
		head
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_908() {
		assert_eq!(
			Solution::middle_node(linked![1, 2, 3, 4, 5]),
			linked![3, 4, 5]
		);
		assert_eq!(
			Solution::middle_node(linked![1, 2, 3, 4, 5, 6]),
			linked![4, 5, 6]
		);
	}
}
