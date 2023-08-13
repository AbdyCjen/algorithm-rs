/**
 * [86] Partition List
 *
 * Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 * You should preserve the original relative order of the nodes in each of the two partitions.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/partition.jpg" style="width: 662px; height: 222px;" />
 * Input: head = [1,4,3,2,5,2], x = 3
 * Output: [1,2,2,4,3,5]
 *
 * <strong class="example">Example 2:
 *
 * Input: head = [2,1], x = 2
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the list is in the range [0, 200].
 *     -100 <= Node.val <= 100
 *     -200 <= x <= 200
 *
 */
pub struct Solution {}
use super::util::linked_list::ListNode;

// submission codes start here
impl Solution {
	pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
		let (mut lnode, mut rnode) = (None, None);
		let (mut l, mut r) = (&mut lnode, &mut rnode);
		while let Some(mut no) = head {
			head = no.next.take();
			if no.val >= x {
				r = &mut r.insert(no).next;
			} else {
				l = &mut l.insert(no).next;
			}
		}
		*l = rnode;
		lnode
	}
}

// submission codes end
