/**
 * [1579] Remove Max Number of Edges to Keep Graph Fully Traversable
 *
 * Alice and Bob have an undirected graph of n nodes and three types of edges:
 *
 *     Type 1: Can be traversed by Alice only.
 *     Type 2: Can be traversed by Bob only.
 *     Type 3: Can be traversed by both Alice and Bob.
 *
 * Given an array edges where edges[i] = [typei, ui, vi] represents a bidirectional edge of type typei between nodes ui and vi, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob. The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.
 * Return the maximum number of edges you can remove, or return -1 if Alice and Bob cannot fully traverse the graph.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex1.png" style="width: 179px; height: 191px;" />
 *
 * Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
 * Output: 2
 * Explanation: If we remove the 2 edges [1,1,2] and [1,1,3]. The graph will still be fully traversable by Alice and Bob. Removing any additional edge will not make it so. So the maximum number of edges we can remove is 2.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex2.png" style="width: 178px; height: 190px;" />
 *
 * Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
 * Output: 0
 * Explanation: Notice that removing any edge will not make the graph fully traversable by Alice and Bob.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex3.png" style="width: 178px; height: 190px;" />
 *
 * Input: n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
 * Output: -1
 * Explanation: In the current graph, Alice cannot reach node 4 from the other nodes. Likewise, Bob cannot reach 1. Therefore it's impossible to make the graph fully traversable.
 *  
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *     1 <= edges.length <= min(10^5, 3 * n * (n - 1) / 2)
 *     edges[i].length == 3
 *     1 <= typei <= 3
 *     1 <= ui < vi <= n
 *     All tuples (typei, ui, vi) are distinct.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
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
		fn connected(set: &[i32]) -> bool {
			set.iter().zip(0..).filter(|(a, b)| *a == b).count() == 1
		}
		let mut sets: [Vec<i32>; 2] = [(0..=n).collect(), (0..=n).collect()];
		edges.push(vec![3, 0, 1]);
		edges.sort_unstable_by_key(|v| -v[0]);

		let mut ans = edges.len() as i32;
		for e in edges {
			if e[0] == 1 {
				if find(&mut sets[0], e[1]) != find(&mut sets[0], e[2]) {
					union(&mut sets[0], e[1], e[2]);
					ans -= 1;
				}
			} else if e[0] == 2 {
				if find(&mut sets[1], e[1]) != find(&mut sets[1], e[2]) {
					union(&mut sets[1], e[1], e[2]);
					ans -= 1;
				}
			} else if find(&mut sets[1], e[1]) != find(&mut sets[1], e[2])
				|| find(&mut sets[0], e[1]) != find(&mut sets[0], e[2])
			{
				union(&mut sets[0], e[1], e[2]);
				union(&mut sets[1], e[1], e[2]);
				ans -= 1;
			}
		}
		if sets.iter().all(|v| connected(v)) {
			ans
		} else {
			-1
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1579() {
		assert_eq!(
			Solution::max_num_edges_to_remove(
				4,
				matrix![[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]]
			),
			0
		);
		assert_eq!(
			Solution::max_num_edges_to_remove(2, matrix![[1, 1, 2], [2, 1, 2], [3, 1, 2]]),
			2
		);
		assert_eq!(
			Solution::max_num_edges_to_remove(4, matrix![[3, 2, 3], [1, 1, 2], [2, 3, 4]]),
			-1
		);
		assert_eq!(
			Solution::max_num_edges_to_remove(
				4,
				matrix![
					[3, 1, 2],
					[3, 2, 3],
					[1, 1, 3],
					[1, 2, 4],
					[1, 1, 2],
					[2, 3, 4]
				]
			),
			2
		);
	}
}
