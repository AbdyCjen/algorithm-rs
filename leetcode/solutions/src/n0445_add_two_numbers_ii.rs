/**
 * [445] Add Two Numbers II
 *
 * You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/sumii-linked-list.jpg" style="width: 523px; height: 342px;" />
 * Input: l1 = [7,2,4,3], l2 = [5,6,4]
 * Output: [7,8,0,7]
 *
 * Example 2:
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [8,0,7]
 *
 * Example 3:
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in each linked list is in the range [1, 100].
 *     0 <= Node.val <= 9
 *     It is guaranteed that the list represents a number that does not have leading zeros.
 *
 *  
 * Follow up: Could you solve it without reversing the input lists?
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
	pub fn add_two_numbers(
		l1: Option<Box<ListNode>>,
		l2: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		fn rev(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
			let mut list = None;
			while let Some(mut no) = head {
				head = std::mem::replace(&mut no.next, list);
				list = Some(no);
			}
			list
		}
		let mut lr = (rev(l1), rev(l2));
		let (mut no, mut flow) = (None, 0);
		while let (Some(mut l1), Some(mut l2)) = lr {
			lr = (std::mem::replace(&mut l1.next, no), l2.next.take());
			l1.val += flow + l2.val;
			flow = l1.val / 10;
			l1.val %= 10;
			no = Some(l1);
		}
		let mut l = lr.0.or(lr.1);
		while let Some(mut ne) = l {
			l = std::mem::replace(&mut ne.next, no);
			ne.val += flow;
			flow = ne.val / 10;
			ne.val %= 10;
			no = Some(ne);
		}
		if flow > 0 {
			Some(Box::new(ListNode { val: 1, next: no }))
		} else {
			no
		}
	}
	pub fn add_two_numbers1(
		l1: Option<Box<ListNode>>,
		l2: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		fn reverse(mut lis: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
			let (mut cnt, mut new_list) = (0, None);
			while let Some(mut l) = lis {
				lis = std::mem::replace(&mut l.next, new_list);
				new_list = Some(l);
				cnt += 1;
			}
			(new_list, cnt)
		}

		let (mut l1, c1) = reverse(l1);
		let (mut l2, c2) = reverse(l2);
		if c2 > c1 {
			std::mem::swap(&mut l1, &mut l2);
		}

		let mut cur_ptr = l1.as_deref_mut();
		let mut flow = 0;
		while let Some(cur) = cur_ptr {
			cur.val += flow;
			if let Some(lis) = l2 {
				cur.val += lis.val;
				flow = cur.val / 10;
				cur.val %= 10;
				l2 = lis.next;
			} else {
				flow = cur.val / 10;
				cur.val %= 10;
			}
			cur_ptr = cur.next.as_deref_mut();
		}
		let (l1, _) = reverse(l1);

		if flow > 0 {
			Some(Box::new(ListNode {
				val: flow,
				next: l1,
			}))
		} else {
			l1
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::linked_list::to_list, *};

	#[test]
	fn test_445() {
		assert_eq!(
			Solution::add_two_numbers(linked![7, 2, 4, 3], linked![5, 6, 4]),
			linked![7, 8, 0, 7]
		);
		assert_eq!(
			Solution::add_two_numbers(linked![2, 4, 3], linked![5, 6, 4]),
			linked![8, 0, 7]
		);
		assert_eq!(
			Solution::add_two_numbers(linked![9, 9, 9], linked![1]),
			linked![1, 0, 0, 0]
		);
	}
}
