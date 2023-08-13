/**
 * [1489] Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree
 *
 * Given a weighted undirected connected graph with n vertices numbered from 0 to n - 1, and an array edges where edges[i] = [ai, bi, weighti] represents a bidirectional and weighted edge between nodes ai and bi. A minimum spanning tree (MST) is a subset of the graph's edges that connects all vertices without cycles and with the minimum possible total edge weight.
 * Find all the critical and pseudo-critical edges in the given graph's minimum spanning tree (MST). An MST edge whose deletion from the graph would cause the MST weight to increase is called a critical edge. On the other hand, a pseudo-critical edge is that which can appear in some MSTs but not all.
 * Note that you can return the indices of the edges in any order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/ex1.png" style="width: 259px; height: 262px;" />
 *
 * Input: n = 5, edges = [[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]]
 * Output: [[0,1],[2,3,4,5]]
 * Explanation: The figure above describes the graph.
 * The following figure shows all the possible MSTs:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/msts.png" style="width: 540px; height: 553px;" />
 * Notice that the two edges 0 and 1 appear in all MSTs, therefore they are critical edges, so we return them in the first list of the output.
 * The edges 2, 3, 4, and 5 are only part of some MSTs, therefore they are considered pseudo-critical edges. We add them to the second list of the output.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/ex2.png" style="width: 247px; height: 253px;" />
 *
 * Input: n = 4, edges = [[0,1,1],[1,2,1],[2,3,1],[0,3,1]]
 * Output: [[],[0,1,2,3]]
 * Explanation: We can observe that since all 4 edges have equal weight, choosing any 3 edges from the given 4 will yield an MST. Therefore all 4 edges are pseudo-critical.
 *
 *  
 * Constraints:
 *
 *     2 <= n <= 100
 *     1 <= edges.length <= min(200, n * (n - 1) / 2)
 *     edges[i].length == 3
 *     0 <= ai < bi < n
 *     1 <= weighti <= 1000
 *     All pairs (ai, bi) are distinct.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		// (w, idx, u, v)
		let mut edges: Vec<_> = (0..)
			.zip(edges)
			.map(|(i, v)| [v[2], i, v[0], v[1]])
			.collect();
		edges.sort_unstable();
		let mut ans = [vec![], vec![]];
		let wei_tot = match Self::kruskal(&edges, 0, &mut (0..n).collect::<Vec<_>>()) {
			None => return ans.into(),
			Some(tot) => tot,
		};
		for i in 0..edges.len() {
			let w = std::mem::replace(&mut edges[i][0], 1001);
			if Self::kruskal(&edges, 0, &mut (0..n).collect::<Vec<_>>()) != Some(wei_tot) {
				ans[0].push(edges[i][1]);
			} else {
				let mut set = (0..n).collect::<Vec<_>>();
				set[edges[i][2] as usize] = edges[i][3];
				if Self::kruskal(&edges, w, &mut set) == Some(wei_tot) {
					ans[1].push(edges[i][1]);
				}
			}
			edges[i][0] = w;
		}
		ans.into()
	}

	fn kruskal(edges: &[[i32; 4]], mut ans: i32, set: &mut [i32]) -> Option<i32> {
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
		for e in edges {
			if e[0] <= 1000 && find(set, e[2]) != find(set, e[3]) {
				union(set, e[2], e[3]);
				ans += e[0];
			}
		}
		if (0..set.len() as i32).all(|x| find(set, x) == find(set, 0)) {
			Some(ans)
		} else {
			None
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1489() {
		assert_eq!(
			Solution::find_critical_and_pseudo_critical_edges(
				5,
				matrix![
					[0, 1, 1],
					[1, 2, 1],
					[2, 3, 2],
					[0, 3, 2],
					[0, 4, 3],
					[3, 4, 3],
					[1, 4, 6]
				]
			),
			matrix![[0, 1], [2, 3, 4, 5]]
		);
		assert_eq!(
			Solution::find_critical_and_pseudo_critical_edges(
				4,
				matrix![[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]]
			),
			matrix![[], [0, 1, 2, 3]]
		);
	}
}
