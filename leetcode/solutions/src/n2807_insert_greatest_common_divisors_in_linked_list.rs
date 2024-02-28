/**
 * [2807] Insert Greatest Common Divisors in Linked List
 *
 * Given the head of a linked list head, in which each node contains an integer value.
 * Between every pair of adjacent nodes, insert a new node with a value equal to the greatest common divisor of them.
 * Return the linked list after insertion.
 * The greatest common divisor of two numbers is the largest positive integer that evenly divides both numbers.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/07/18/ex1_copy.png" style="width: 641px; height: 181px;" />
 * Input: head = [18,6,10,3]
 * Output: [18,6,6,2,10,1,3]
 * Explanation: The 1^st diagram denotes the initial linked list and the 2^nd diagram denotes the linked list after inserting the new nodes (nodes in blue are the inserted nodes).
 * - We insert the greatest common divisor of 18 and 6 = 6 between the 1^st and the 2^nd nodes.
 * - We insert the greatest common divisor of 6 and 10 = 2 between the 2^nd and the 3^rd nodes.
 * - We insert the greatest common divisor of 10 and 3 = 1 between the 3^rd and the 4^th nodes.
 * There are no more adjacent nodes, so we return the linked list.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/07/18/ex2_copy1.png" style="width: 51px; height: 191px;" />
 * Input: head = [7]
 * Output: [7]
 * Explanation: The 1^st diagram denotes the initial linked list and the 2^nd diagram denotes the linked list after inserting the new nodes.
 * There are no pairs of adjacent nodes, so we return the initial linked list.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [1, 5000].
 *     1 <= Node.val <= 1000
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
	fn gcd(a: i32, b: i32) -> i32 {
		match a % b {
			0 => b,
			c => Self::gcd(b, c),
		}
	}
	pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut head = head?;
		let mut cur = &mut *head;
		while let Some(nn) = cur.next.take() {
			let new = Box::new(ListNode::new(Self::gcd(cur.val, nn.val)));
			cur = cur.next.insert(new).next.insert(nn);
		}
		head.into()
	}
}

// submission codes end
