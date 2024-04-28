/**
 * [684] Redundant Connection
 *
 * In this problem, a tree is an undirected graph that is connected and has no cycles.
 * You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.
 * Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/reduntant1-1-graph.jpg" style="width: 222px; height: 222px;" />
 * Input: edges = [[1,2],[1,3],[2,3]]
 * Output: [2,3]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/reduntant1-2-graph.jpg" style="width: 382px; height: 222px;" />
 * Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
 * Output: [1,4]
 *
 *  
 * Constraints:
 *
 *     n == edges.length
 *     3 <= n <= 1000
 *     edges[i].length == 2
 *     1 <= ai < bi <= edges.length
 *     ai != bi
 *     There are no repeated edges.
 *     The given graph is connected.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
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
		let mut set = (0..=edges.len() as i32).collect::<Vec<i32>>();
		for e in edges {
			if find(&mut set, e[0]) == find(&mut set, e[1]) {
				return e;
			}
			union(&mut set, e[0], e[1]);
		}
		vec![]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_684() {}
}
