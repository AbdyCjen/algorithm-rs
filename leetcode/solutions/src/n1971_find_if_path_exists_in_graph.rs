/**
 * [1971] Find if Path Exists in Graph
 *
 * There is a bi-directional graph with n vertices, where each vertex is labeled from 0 to n - 1 (inclusive). The edges in the graph are represented as a 2D integer array edges, where each edges[i] = [ui, vi] denotes a bi-directional edge between vertex ui and vertex vi. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.
 * You want to determine if there is a valid path that exists from vertex source to vertex destination.
 * Given edges and the integers n, source, and destination, return true if there is a valid path from source to destination, or false otherwise.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/14/validpath-ex1.png" style="width: 141px; height: 121px;" />
 * Input: n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
 * Output: true
 * Explanation: There are two paths from vertex 0 to vertex 2:
 * - 0 &rarr; 1 &rarr; 2
 * - 0 &rarr; 2
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/14/validpath-ex2.png" style="width: 281px; height: 141px;" />
 * Input: n = 6, edges = [[0,1],[0,2],[3,5],[5,4],[4,3]], source = 0, destination = 5
 * Output: false
 * Explanation: There is no path from vertex 0 to vertex 5.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 2 * 10^5
 *     0 <= edges.length <= 2 * 10^5
 *     edges[i].length == 2
 *     0 <= ui, vi <= n - 1
 *     ui != vi
 *     0 <= source, destination <= n - 1
 *     There are no duplicate edges.
 *     There are no self edges.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
		fn union(s: &mut [i32], a: i32, b: i32) {
			let a = find(s, a);
			let b = find(s, b);
			if a != b {
				s[a as usize] = b;
			}
		}
		fn find(s: &mut [i32], a: i32) -> i32 {
			if s[a as usize] != a {
				s[a as usize] = find(s, s[a as usize]);
			}
			s[a as usize]
		}
		let mut set = (0..n).collect::<Vec<_>>();
		for e in edges {
			union(set.as_mut(), e[0], e[1]);
		}
		find(set.as_mut(), source) == find(set.as_mut(), destination)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1971() {}
}
