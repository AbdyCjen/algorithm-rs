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
		l: Option<Box<ListNode>>,
		r: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		let mut list = None;
		let mut cur = &mut list;
		let mut lr = (l, r);
		while let (Some(mut ln), Some(mut rn)) = lr {
			if ln.val > rn.val {
				lr = (rn.next.take(), Some(ln));
				cur = &mut cur.insert(rn).next;
			} else {
				lr = (ln.next.take(), Some(rn));
				cur = &mut cur.insert(ln).next;
			}
		}
		*cur = lr.0.or(lr.1);
		list
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
