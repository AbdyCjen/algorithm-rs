/**
 * [206] Reverse Linked List
 *
 * Given the head of a singly linked list, reverse the list, and return the reversed list.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [5,4,3,2,1]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg" style="width: 182px; height: 222px;" />
 * Input: head = [1,2]
 * Output: [2,1]
 *
 * <strong class="example">Example 3:
 *
 * Input: head = []
 * Output: []
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is the range [0, 5000].
 *     -5000 <= Node.val <= 5000
 *
 *  
 * Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
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
	pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut new = None;
		while let Some(mut no) = head {
			head = std::mem::replace(&mut no.next, new);
			new = Some(no);
		}
		new
	}
}

// submission codes end
