/**
 * [2359] Find Closest Node to Given Two Nodes
 *
 * You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.
 * The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from i, then edges[i] == -1.
 * You are also given two integers node1 and node2.
 * Return the index of the node that can be reached from both node1 and node2, such that the maximum between the distance from node1 to that node, and from node2 to that node is minimized. If there are multiple answers, return the node with the smallest index, and if no possible answer exists, return -1.
 * Note that edges may contain cycles.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-2.png" style="width: 321px; height: 161px;" />
 * Input: edges = [2,2,3,-1], node1 = 0, node2 = 1
 * Output: 2
 * Explanation: The distance from node 0 to node 2 is 1, and the distance from node 1 to node 2 is 1.
 * The maximum of those two distances is 1. It can be proven that we cannot get a node with a smaller maximum distance than 1, so we return node 2.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-4.png" style="width: 195px; height: 161px;" />
 * Input: edges = [1,2,-1], node1 = 0, node2 = 2
 * Output: 2
 * Explanation: The distance from node 0 to node 2 is 2, and the distance from node 2 to itself is 0.
 * The maximum of those two distances is 2. It can be proven that we cannot get a node with a smaller maximum distance than 2, so we return node 2.
 *
 *  
 * Constraints:
 *
 *     n == edges.length
 *     2 <= n <= 10^5
 *     -1 <= edges[i] < n
 *     edges[i] != i
 *     0 <= node1, node2 < n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
		let mut visited = vec![-1; edges.len()];
		visited[node1 as usize] = 0;
		let mut cur = node1;
		for st in 1.. {
			if edges[cur as usize] != -1 && visited[edges[cur as usize] as usize] < 0 {
				cur = edges[cur as usize];
				visited[cur as usize] = st;
			} else {
				break;
			}
		}
		let (mut min_dist, mut ans) = (i32::MAX, -1);
		cur = node2;
		for st in 0..edges.len() as i32 {
			if visited[cur as usize] >= 0 {
				let d = visited[cur as usize].max(st);
				match d.cmp(&min_dist) {
					std::cmp::Ordering::Less => {
						min_dist = d;
						ans = cur;
					}
					std::cmp::Ordering::Equal => ans = ans.min(cur),
					_ => {}
				}
			}
			if edges[cur as usize] >= 0 {
				cur = edges[cur as usize];
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2359() {
		assert_eq!(
			//5 8 1 4 9 1
			//6 4 9 1
			Solution::closest_meeting_node(vec![4, 4, 8, -1, 9, 8, 4, 4, 1, 1], 5, 6),
			1
		);
		assert_eq!(
			Solution::closest_meeting_node(vec![5, 3, 1, 0, 2, 4, 5], 3, 2),
			3
		);
		assert_eq!(
			Solution::closest_meeting_node(vec![4, 3, 0, 5, 3, -1], 4, 0),
			4
		);
		assert_eq!(
			Solution::closest_meeting_node(vec![5, -1, 3, 4, 5, 6, -1, -1, 4, 3], 0, 0),
			0
		);
	}
}
