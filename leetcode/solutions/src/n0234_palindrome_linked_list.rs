/**
 * [234] Palindrome Linked List
 *
 * Given the head of a singly linked list, return true if it is a <span data-keyword="palindrome-sequence">palindrome</span> or false otherwise.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal1linked-list.jpg" style="width: 422px; height: 62px;" />
 * Input: head = [1,2,2,1]
 * Output: true
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal2linked-list.jpg" style="width: 182px; height: 62px;" />
 * Input: head = [1,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [1, 10^5].
 *     0 <= Node.val <= 9
 *
 *  
 * Follow up: Could you do it in O(n) time and O(1) space?
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
	pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
		let mut cur = &head;
		let mut nums = vec![];
		while let Some(no) = cur {
			nums.push(no.val);
			cur = &no.next;
		}
		let len = nums.len();
		nums[..len / 2]
			.iter()
			.eq(nums[(len + 1) / 2..].iter().rev())
	}
	pub fn is_palindrome1(mut head: Option<Box<ListNode>>) -> bool {
		let len = std::iter::successors(head.as_ref(), |no| no.next.as_ref()).count();
		let mut cur = &mut head;
		for _ in 0..len / 2 {
			cur = &mut cur.as_mut().unwrap().next;
		}
		let mut new = cur.take();
		let mut tail = None;
		while let Some(mut ne) = new {
			new = ne.next.take();
			ne.next = tail;
			tail = Some(ne);
		}
		while let (Some(mut h), Some(mut t)) = (head, tail) {
			if h.val != t.val {
				return false;
			}
			head = h.next.take();
			tail = t.next.take();
		}
		true
	}
}

// submission codes end
