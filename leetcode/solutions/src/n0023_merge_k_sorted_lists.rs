/**
 * [23] Merge k Sorted Lists
 *
 * You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
 * Merge all the linked-lists into one sorted linked-list and return it.
 *  
 * Example 1:
 *
 * Input: lists = [[1,4,5],[1,3,4],[2,6]]
 * Output: [1,1,2,3,4,4,5,6]
 * Explanation: The linked-lists are:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 *
 * Example 2:
 *
 * Input: lists = []
 * Output: []
 *
 * Example 3:
 *
 * Input: lists = [[]]
 * Output: []
 *
 *  
 * Constraints:
 *
 *  k == lists.length
 *  0 <= k <= 10^4
 *  0 <= lists[i].length <= 500
 *  -10^4 <= lists[i][j] <= 10^4
 *  lists[i] is sorted in ascending order.
 *  The sum of lists[i].length won't exceed 10^4.
 *
 */
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
use super::util::linked_list::ListNode;
pub struct Solution {}

use std::cmp::Ordering;
impl PartialOrd for ListNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for ListNode {
	fn cmp(&self, other: &Self) -> Ordering { other.val.cmp(&self.val) }
}

impl Solution {
	pub fn merge_k_lists_01(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
		let mut head = None;
		let mut cur = &mut head;

		let mut bh: std::collections::BinaryHeap<_> = lists.into_iter().flatten().collect();
		while let Some(no) = bh.pop() {
			cur = &mut cur.insert(no).next;
			if let Some(no) = cur.take() {
				bh.push(no);
			}
		}
		head
	}

	pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
		let mut dq = std::collections::VecDeque::from(lists);
		loop {
			match (dq.pop_back(), dq.pop_back()) {
				(Some(l), Some(r)) => dq.push_front(Self::merge_list((l, r))),
				(l, _) => break l.flatten(),
			}
		}
	}

	fn merge_list(mut lr: (Option<Box<ListNode>>, Option<Box<ListNode>>)) -> Option<Box<ListNode>> {
		let mut list = None;
		let mut cur = &mut list;
		while let (Some(mut l), Some(mut r)) = lr {
			if l.val < r.val {
				lr = (l.next.take(), Some(r));
				cur = &mut cur.insert(l).next;
			} else {
				lr = (Some(l), r.next.take());
				cur = &mut cur.insert(r).next;
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
	fn test_23() {
		assert_eq!(
			Solution::merge_k_lists(vec![linked![1, 4, 5], linked![1, 3, 4], linked![2, 6]]),
			linked![1, 1, 2, 3, 4, 4, 5, 6]
		);
		assert_eq!(Solution::merge_k_lists(vec![]), linked![]);
		assert_eq!(Solution::merge_k_lists(vec![linked![]]), linked![]);
	}
}
