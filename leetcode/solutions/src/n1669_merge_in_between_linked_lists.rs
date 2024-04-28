/**
 * [1669] Merge In Between Linked Lists
 *
 * You are given two linked lists: list1 and list2 of sizes n and m respectively.
 * Remove list1's nodes from the a^th node to the b^th node, and put list2 in their place.
 * The blue edges and nodes in the following figure indicate the result:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/fig1.png" style="height: 130px; width: 504px;" />
 * Build the result list and return its head.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2024/03/01/ll.png" style="width: 609px; height: 210px;" />
 * Input: list1 = [10,1,13,6,9,5], a = 3, b = 4, list2 = [1000000,1000001,1000002]
 * Output: [10,1,13,1000000,1000001,1000002,5]
 * Explanation: We remove the nodes 3 and 4 and put the entire list2 in their place. The blue edges and nodes in the above figure indicate the result.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/merge_linked_list_ex2.png" style="width: 463px; height: 140px;" />
 * Input: list1 = [0,1,2,3,4,5,6], a = 2, b = 5, list2 = [1000000,1000001,1000002,1000003,1000004]
 * Output: [0,1,1000000,1000001,1000002,1000003,1000004,6]
 * Explanation: The blue edges and nodes in the above figure indicate the result.
 *
 *  
 * Constraints:
 *
 *     3 <= list1.length <= 10^4
 *     1 <= a <= b < list1.length - 1
 *     1 <= list2.length <= 10^4
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
	pub fn merge_in_between(
		mut list1: Option<Box<ListNode>>,
		a: i32,
		b: i32,
		list2: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		let mut i = 0;
		let mut cur = list1.as_mut().unwrap();
		let mut tail = None;
		while let Some(mut ne) = cur.next.take() {
			i += 1;
			if i > b {
				tail = Some(ne);
				break;
			} else if i >= a {
				cur.next = ne.next.take();
			} else {
				cur = cur.next.insert(ne);
			}
		}
		cur.next = list2;
		while let Some(ne) = cur.next.take() {
			cur = cur.next.insert(ne);
		}
		cur.next = tail;

		list1
	}
}

// submission codes end
