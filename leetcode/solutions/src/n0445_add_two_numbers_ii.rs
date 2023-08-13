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
impl Solution {
	pub fn add_two_numbers(
		l1: Option<Box<ListNode>>,
		l2: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		fn rev(mut lis: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
			let mut no = None;
			while let Some(mut l) = lis {
				lis = std::mem::replace(&mut l.next, no);
				no = Some(l);
			}
			no
		}
		let mut lr = (rev(l1), rev(l2));
		let (mut no, mut flow) = (None, 0);
		while let (Some(mut l), mut r) | (mut r, Some(mut l)) = lr {
			if let Some(rr) = r {
				l.val += rr.val;
				r = rr.next;
			}
			lr = (std::mem::replace(&mut l.next, no), r);
			l.val += flow;
			flow = l.val / 10;
			l.val %= 10;
			no = Some(l);
		}
		if flow > 0 {
			no = Some(Box::new(ListNode { val: 1, next: no }));
		}
		no
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
