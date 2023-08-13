/**
 * [725] Split Linked List in Parts
 *
 * Given the head of a singly linked list and an integer k, split the linked list into k consecutive linked list parts.
 * The length of each part should be as equal as possible: no two parts should have a size differing by more than one. This may lead to some parts being null.
 * The parts should be in the order of occurrence in the input list, and parts occurring earlier should always have a size greater than or equal to parts occurring later.
 * Return an array of the k parts.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/split1-lc.jpg" style="width: 400px; height: 134px;" />
 * Input: head = [1,2,3], k = 5
 * Output: [[1],[2],[3],[],[]]
 * Explanation:
 * The first element output[0] has output[0].val = 1, output[0].next = null.
 * The last element output[4] is null, but its string representation as a ListNode is [].
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/split2-lc.jpg" style="width: 600px; height: 60px;" />
 * Input: head = [1,2,3,4,5,6,7,8,9,10], k = 3
 * Output: [[1,2,3,4],[5,6,7],[8,9,10]]
 * Explanation:
 * The input has been split into consecutive parts with size difference at most 1, and earlier parts are a larger size than the later parts.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [0, 1000].
 *     0 <= Node.val <= 1000
 *     1 <= k <= 50
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
	pub fn split_list_to_parts(
		mut head: Option<Box<ListNode>>,
		k: i32,
	) -> Vec<Option<Box<ListNode>>> {
		let mut len = std::iter::successors(head.as_ref(), |no| no.next.as_ref()).count() as i32;
		let mut ans = vec![];
		for i in (1..=k).rev() {
			let mut h = None;
			let mut cur = &mut h;
			let l = (len + i - 1) / i;
			for _ in 0..l {
				cur = &mut cur.insert(head.unwrap()).next;
				head = cur.take();
			}
			len -= l;
			ans.push(h);
		}
		ans
	}
}

// submission codes end
