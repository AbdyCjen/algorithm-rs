/**
 * [148] Sort List
 *
 * Given the head of a linked list, return the list after sorting it in ascending order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_1.jpg" style="width: 450px; height: 194px;" />
 * Input: head = [4,2,1,3]
 * Output: [1,2,3,4]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_2.jpg" style="width: 550px; height: 184px;" />
 * Input: head = [-1,5,3,4,0]
 * Output: [-1,0,3,4,5]
 *
 * <strong class="example">Example 3:
 *
 * Input: head = []
 * Output: []
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [0, 5 * 10^4].
 *     -10^5 <= Node.val <= 10^5
 *
 *  
 * Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?
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
	pub fn sort_list(mut lis: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut nums: Vec<_> = std::iter::successors(lis.as_ref(), |no| no.next.as_ref())
			.map(|no| no.val)
			.collect();
		nums.sort_unstable();
		let mut cur = &mut lis;
		let mut it = nums.into_iter();
		while let (Some(no), Some(n)) = (cur, it.next()) {
			no.val = n;
			cur = &mut no.next;
		}
		lis
	}
	pub fn sort_list1(lis: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut len = 0;
		let mut cur = &lis;
		while let Some(n) = cur {
			cur = &n.next;
			len += 1;
		}
		Self::sort(lis, len)
	}

	fn sort(mut lis: Option<Box<ListNode>>, len: i32) -> Option<Box<ListNode>> {
		if len <= 1 {
			return lis;
		}
		let mut cnt = (len + 1) / 2;
		let mut cur = &mut lis;
		while let Some(no) = cur {
			cur = &mut no.next;
			cnt -= 1;
			if cnt == 0 {
				break;
			}
		}
		let l = Self::sort(cur.take(), len / 2);
		let r = Self::sort(lis, (len + 1) / 2);
		Self::merge(l, r)
	}

	fn merge(l: Option<Box<ListNode>>, r: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut lr = (l, r);
		let mut lis = None;
		let mut cur = &mut lis;
		while let (Some(mut l), Some(mut r)) = lr {
			if l.val > r.val {
				lr = (Some(l), r.next.take());
				cur = &mut cur.insert(r).next;
			} else {
				lr = (l.next.take(), Some(r));
				cur = &mut cur.insert(l).next;
			}
		}
		*cur = lr.0.or(lr.1);
		lis
	}
}

// submission codes end
