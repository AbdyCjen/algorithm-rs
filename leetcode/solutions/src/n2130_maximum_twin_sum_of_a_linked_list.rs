/**
 * [2130] Maximum Twin Sum of a Linked List
 *
 * In a linked list of size n, where n is even, the i^th node (0-indexed) of the linked list is known as the twin of the (n-1-i)^th node, if 0 <= i <= (n / 2) - 1.
 *
 *     For example, if n = 4, then node 0 is the twin of node 3, and node 1 is the twin of node 2. These are the only nodes with twins for n = 4.
 *
 * The twin sum is defined as the sum of a node and its twin.
 * Given the head of a linked list with even length, return the maximum twin sum of the linked list.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/03/eg1drawio.png" style="width: 250px; height: 70px;" />
 * Input: head = [5,4,2,1]
 * Output: 6
 * Explanation:
 * Nodes 0 and 1 are the twins of nodes 3 and 2, respectively. All have twin sum = 6.
 * There are no other nodes with twins in the linked list.
 * Thus, the maximum twin sum of the linked list is 6.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/03/eg2drawio.png" style="width: 250px; height: 70px;" />
 * Input: head = [4,2,2,3]
 * Output: 7
 * Explanation:
 * The nodes with twins present in this linked list are:
 * - Node 0 is the twin of node 3 having a twin sum of 4 + 3 = 7.
 * - Node 1 is the twin of node 2 having a twin sum of 2 + 2 = 4.
 * Thus, the maximum twin sum of the linked list is max(7, 4) = 7.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/03/eg3drawio.png" style="width: 200px; height: 88px;" />
 * Input: head = [1,100000]
 * Output: 100001
 * Explanation:
 * There is only one node with a twin in the linked list having twin sum of 1 + 100000 = 100001.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is an even integer in the range [2, 10^5].
 *     1 <= Node.val <= 10^5
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
	pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
		fn rev(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
			let mut no = None;
			while let Some(mut ne) = head {
				head = std::mem::replace(&mut ne.next, no);
				no = Some(ne);
			}
			no
		}
		let len = std::iter::successors(head.as_ref(), |no| no.next.as_ref()).count() as i32;
		let mut cur = &mut head;
		for _ in 0..len / 2 {
			cur = &mut cur.as_mut().unwrap().next;
		}
		let mut ht = (rev(cur.take()), head);
		let mut ans = 0;
		while let (Some(t), Some(h)) = ht {
			ans = ans.max(h.val + t.val);
			ht = (h.next, t.next);
		}
		ans
	}
}

// submission codes end
