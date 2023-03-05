/**
 * [1129] Shortest Path with Alternating Colors
 *
 * You are given an integer n, the number of nodes in a directed graph where the nodes are labeled from 0 to n - 1. Each edge is red or blue in this graph, and there could be self-edges and parallel edges.
 * You are given two arrays redEdges and blueEdges where:
 *
 *     redEdges[i] = [ai, bi] indicates that there is a directed red edge from node ai to node bi in the graph, and
 *     blueEdges[j] = [uj, vj] indicates that there is a directed blue edge from node uj to node vj in the graph.
 *
 * Return an array answer of length n, where each answer[x] is the length of the shortest path from node 0 to node x such that the edge colors alternate along the path, or -1 if such a path does not exist.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 3, redEdges = [[0,1],[1,2]], blueEdges = []
 * Output: [0,1,-1]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 3, redEdges = [[0,1]], blueEdges = [[2,1]]
 * Output: [0,1,-1]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 100
 *     0 <= redEdges.length, blueEdges.length <= 400
 *     redEdges[i].length == blueEdges[j].length == 2
 *     0 <= ai, bi, uj, vj < n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn shortest_alternating_paths(
		n: i32,
		red_edges: Vec<Vec<i32>>,
		blue_edges: Vec<Vec<i32>>,
	) -> Vec<i32> {
		let map = std::collections::HashMap::new();
		let mut neigs = [map.clone(), map];

		for (neig, edges) in neigs.iter_mut().zip([red_edges, blue_edges]) {
			for e in edges {
				neig.entry(e[0]).or_insert_with(Vec::new).push(e[1]);
			}
		}

		let mut st = vec![(0, 0), (1, 0)];
		let mut ans = vec![[i32::MAX; 2]; n as usize];
		ans[0] = [0; 2];
		let mut dist = 1;
		while !st.is_empty() {
			for (c, i) in std::mem::take(&mut st) {
				let c = c ^ 1;
				for &j in neigs[c].get(&i).unwrap_or(&vec![]) {
					if ans[j as usize][c] > dist {
						ans[j as usize][c] = dist;
						st.push((c, j));
					}
				}
			}
			dist += 1;
		}

		ans.into_iter()
			.map(|[a, b]| match a.min(b) {
				i32::MAX => -1,
				i => i,
			})
			.collect()
	}
}

// submission codes end
