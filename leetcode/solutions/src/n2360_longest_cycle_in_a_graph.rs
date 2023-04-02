/**
 * [2360] Longest Cycle in a Graph
 *
 * You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.
 * The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from node i, then edges[i] == -1.
 * Return the length of the longest cycle in the graph. If no cycle exists, return -1.
 * A cycle is a path that starts and ends at the same node.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/08/graph4drawio-5.png" style="width: 335px; height: 191px;" />
 * Input: edges = [3,3,4,2,3]
 * Output: 3
 * Explanation: The longest cycle in the graph is the cycle: 2 -> 4 -> 3 -> 2.
 * The length of this cycle is 3, so 3 is returned.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-1.png" style="width: 171px; height: 161px;" />
 * Input: edges = [2,-1,3,1]
 * Output: -1
 * Explanation: There are no cycles in this graph.
 *
 *  
 * Constraints:
 *
 *     n == edges.length
 *     2 <= n <= 10^5
 *     -1 <= edges[i] < n
 *     edges[i] != i
 *
 */
pub struct Solution {}

impl Solution {
	pub fn longest_cycle(edges: Vec<i32>) -> i32 {
		fn find(set: &mut [i32], x: i32) -> i32 {
			if set[x as usize] != x {
				set[x as usize] = find(set, set[x as usize]);
			}
			set[x as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}
		let mut set: Vec<_> = (0..edges.len() as i32).collect();
		let mut ans = -1;
		for (&t, f) in edges.iter().zip(0..).filter(|(&t, _)| t != -1) {
			if find(&mut set, f) == find(&mut set, t) {
				let (mut l, mut cur) = (1, t);
				while edges[cur as usize] != t {
					l += 1;
					cur = edges[cur as usize];
				}
				ans = ans.max(l);
			} else {
				union(&mut set, t, f);
			}
		}
		ans
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2360() {
		assert_eq!(Solution::longest_cycle(vec![-1, 4, -1, 2, 0, 4]), -1);
		assert_eq!(Solution::longest_cycle(vec![3, 4, 0, 2, -1, 2]), 3);
		assert_eq!(Solution::longest_cycle(vec![1, 2, 0]), 3);
		assert_eq!(Solution::longest_cycle(vec![2, -1, 3, 1]), -1);
		assert_eq!(Solution::longest_cycle(vec![3, 3, 4, 2, 3]), 3);
	}
}
