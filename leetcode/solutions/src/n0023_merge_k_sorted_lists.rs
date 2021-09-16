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
use std::cmp::{Ord, Ordering, PartialOrd};
impl PartialOrd for ListNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for ListNode {
	fn cmp(&self, other: &Self) -> Ordering { other.val.cmp(&self.val) }
}

use std::collections::BinaryHeap;
#[allow(dead_code)]
impl Solution {
	pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
		let mut dummy = ListNode::new(-1);
		let mut cur_ptr = &mut dummy;

		let mut bh: BinaryHeap<_> = lists.into_iter().flatten().collect();
		while let Some(next_no) = bh.pop() {
			cur_ptr.next = Some(next_no);
			cur_ptr = cur_ptr.next.as_mut()?;
			if let Some(no) = cur_ptr.next.take() {
				bh.push(no);
			}
		}
		dummy.next
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
