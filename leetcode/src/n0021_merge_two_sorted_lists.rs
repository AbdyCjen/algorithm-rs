/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new sorted list. The new list should be made by splicing together the nodes of the first two lists.
 * Example:
 *
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 *
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
	pub fn merge_two_lists(
		mut l: Option<Box<ListNode>>,
		mut r: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		let mut dummy = ListNode::new(-1);
		let mut cur_ptr = &mut dummy;
		while let (Some(lnext), Some(rnext)) = (&mut l, &mut r) {
			if lnext.val > rnext.val {
				std::mem::swap(lnext, rnext);
			}
			cur_ptr.next = l.take();
			cur_ptr = cur_ptr.next.as_mut().unwrap();
			l = cur_ptr.next.take();
		}
		cur_ptr.next = l.or(r);
		dummy.next
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::linked_list::to_list, *};

	#[test]
	fn test_21() {
		assert_eq!(
			Solution::merge_two_lists(linked![1, 2, 4], linked![1, 3, 4]),
			linked![1, 1, 2, 3, 4, 4]
		);
		assert_eq!(
			Solution::merge_two_lists(linked![2], linked![1]),
			linked![1, 2]
		);
		assert_eq!(Solution::merge_two_lists(None, None), None);
		assert_eq!(
			Solution::merge_two_lists(linked![1, 2, 4], None),
			linked![1, 2, 4]
		);
	}
}
