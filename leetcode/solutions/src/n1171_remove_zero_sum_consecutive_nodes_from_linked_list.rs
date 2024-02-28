/**
 * [1171] Remove Zero Sum Consecutive Nodes from Linked List
 *
 * Given the head of a linked list, we repeatedly delete consecutive sequences of nodes that sum to 0 until there are no such sequences.
 *
 * After doing so, return the head of the final linked list.  You may return any such answer.
 *  
 * (Note that in the examples below, all sequences are serializations of ListNode objects.)
 * <strong class="example">Example 1:
 *
 * Input: head = [1,2,-3,3,1]
 * Output: [3,1]
 * Note: The answer [1,2,1] would also be accepted.
 *
 * <strong class="example">Example 2:
 *
 * Input: head = [1,2,3,-3,4]
 * Output: [1,2,4]
 *
 * <strong class="example">Example 3:
 *
 * Input: head = [1,2,3,-3,-2]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 *     The given linked list will contain between 1 and 1000 nodes.
 *     Each node in the linked list has -1000 <= node.val <= 1000.
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
	pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let cur = &mut head;
		let mut s = 0;
		let mut sum = std::collections::HashMap::new();
		sum.insert(0, 1);
		let mut rev: Option<Box<ListNode>> = None;
		while let Some(mut no) = cur.take() {
			s += no.val;
			*cur = no.next.take();
			no.next = rev;
			rev = Some(no);
			let e = sum.entry(s).or_insert(0);
			*e += 1;
			if *e > 1 {
				let target = s;
				while target != s || sum[&target] > 1 {
					*sum.get_mut(&s).unwrap() -= 1;
					let mut rno = rev.unwrap();
					s -= rno.val;
					rev = rno.next.take();
				}
			}
		}
		while let Some(mut no) = rev {
			rev = std::mem::replace(&mut no.next, head);
			head = Some(no);
		}
		head
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::linked_list::to_list, *};

	#[test]
	fn test_1171() {
		assert_eq!(
			Solution::remove_zero_sum_sublists(linked![1, 2, 3, -3, -2]),
			linked![1]
		);
		assert_eq!(
			Solution::remove_zero_sum_sublists(linked![1, 2, -3, 3, 1]),
			linked![3, 1]
		);
		assert_eq!(
			Solution::remove_zero_sum_sublists(linked![1, 2, 3, -3, 4]),
			linked![1, 2, 4]
		);
	}
}
