/**
 * [2385] Amount of Time for Binary Tree to Be Infected
 *
 * You are given the root of a binary tree with unique values, and an integer start. At minute 0, an infection starts from the node with value start.
 * Each minute, a node becomes infected if:
 *
 *     The node is currently uninfected.
 *     The node is adjacent to an infected node.
 *
 * Return the number of minutes needed for the entire tree to be infected.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/25/image-20220625231744-1.png" style="width: 400px; height: 306px;" />
 * Input: root = [1,5,3,null,4,10,6,9,2], start = 3
 * Output: 4
 * Explanation: The following nodes are infected during:
 * - Minute 0: Node 3
 * - Minute 1: Nodes 1, 10 and 6
 * - Minute 2: Node 5
 * - Minute 3: Node 4
 * - Minute 4: Nodes 9 and 2
 * It takes 4 minutes for the whole tree to be infected so we return 4.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/25/image-20220625231812-2.png" style="width: 75px; height: 66px;" />
 * Input: root = [1], start = 1
 * Output: 0
 * Explanation: At minute 0, the only node in the tree is infected so we return 0.
 *
 *  
 * Constraints:
 *
 *     The number of nodes in the tree is in the range [1, 10^5].
 *     1 <= Node.val <= 10^5
 *     Each node has a unique value.
 *     A node with a value of start exists in the tree.
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

// submission codes start here

use std::{cell::RefCell, rc::Rc};
impl Solution {
	pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
		Self::solve(&root, start).0.unwrap()
	}

	// (None, dep)
	// (Some(ans), dist_to_start)
	fn solve(cur: &Option<Rc<RefCell<TreeNode>>>, start: i32) -> (Option<i32>, i32) {
		match cur.as_deref().map(RefCell::borrow) {
			None => (None, 0),
			Some(cur) if cur.val == start => (
				Some(
					Self::solve(&cur.left, start)
						.1
						.max(Self::solve(&cur.right, start).1),
				),
				0,
			),
			Some(cur) => match (
				Self::solve(&cur.left, start),
				Self::solve(&cur.right, start),
			) {
				((Some(a), dist), (_, dep)) | ((_, dep), (Some(a), dist)) => {
					(Some(a.max(dist + 1 + dep)), dist + 1)
				}
				((_, d1), (_, d2)) => (None, d1.max(d2) + 1),
			},
		}
	}
}

// submission codes end
