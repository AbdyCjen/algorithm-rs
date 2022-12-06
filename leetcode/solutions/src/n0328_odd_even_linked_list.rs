/**
 * [328] Odd Even Linked List
 *
 * Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.
 * The first node is considered odd, and the second node is even, and so on.
 * Note that the relative order inside both the even and odd groups should remain as it was in the input.
 * You must solve the problem in O(1) extra space complexity and O(n) time complexity.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/oddeven-linked-list.jpg" style="width: 300px; height: 123px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [1,3,5,2,4]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/oddeven2-linked-list.jpg" style="width: 500px; height: 142px;" />
 * Input: head = [2,1,3,5,6,4,7]
 * Output: [2,3,6,7,1,5,4]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the linked list is in the range [0, 10^4].
 *     -10^6 <= Node.val <= 10^6
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
	pub fn odd_even_list(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let (mut odd, mut even) = (None, None);
		let (mut cur_o, mut cur_e) = (&mut odd, &mut even);

		let mut is_even = false;
		while let Some(mut no) = list {
			list = no.next.take();
			if is_even {
				cur_e = &mut cur_e.insert(no).next;
			} else {
				cur_o = &mut cur_o.insert(no).next;
			}
			is_even ^= true;
		}
		*cur_o = even;
		odd
	}
	pub fn odd_even_list_1(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut cur = &mut list;
		let mut even = None;
		let mut even_cur = &mut even;
		loop {
			match cur {
				Some(no) => match no.next.take() {
					Some(mut no_ev) => {
						no.next = no_ev.next.take();
						cur = &mut no.next;
						even_cur = &mut even_cur.insert(no_ev).next;
					}
					_ => {
						no.next = even;
						break list;
					}
				},
				_ => {
					*cur = even;
					break list;
				}
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::linked_list::to_list, *};

	#[test]
	fn test_328() {
		assert_eq!(
			Solution::odd_even_list(linked![2, 1, 3, 5, 6, 4, 7]),
			linked![2, 3, 6, 7, 1, 5, 4]
		);
		assert_eq!(
			Solution::odd_even_list(linked!(1, 2, 3, 4, 5)),
			linked!(1, 3, 5, 2, 4)
		);
	}
}
