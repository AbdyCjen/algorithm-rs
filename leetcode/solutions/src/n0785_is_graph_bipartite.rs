/**
 * [785] Is Graph Bipartite?
 *
 * There is an undirected graph with n nodes, where each node is numbered between 0 and n - 1. You are given a 2D array graph, where graph[u] is an array of nodes that node u is adjacent to. More formally, for each v in graph[u], there is an undirected edge between node u and node v. The graph has the following properties:
 *
 *     There are no self-edges (graph[u] does not contain u).
 *     There are no parallel edges (graph[u] does not contain duplicate values).
 *     If v is in graph[u], then u is in graph[v] (the graph is undirected).
 *     The graph may not be connected, meaning there may be two nodes u and v such that there is no path between them.
 *
 * A graph is bipartite if the nodes can be partitioned into two independent sets A and B such that every edge in the graph connects a node in set A and a node in set B.
 * Return true if and only if it is bipartite.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/21/bi2.jpg" style="width: 222px; height: 222px;" />
 * Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
 * Output: false
 * Explanation: There is no way to partition the nodes into two independent sets such that every edge connects a node in one and a node in the other.
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/21/bi1.jpg" style="width: 222px; height: 222px;" />
 * Input: graph = [[1,3],[0,2],[1,3],[0,2]]
 * Output: true
 * Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.
 *  
 * Constraints:
 *
 *     graph.length == n
 *     1 <= n <= 100
 *     0 <= graph[u].length < n
 *     0 <= graph[u][i] <= n - 1
 *     graph[u] does not contain u.
 *     All the values of graph[u] are unique.
 *     If graph[u] contains v, then graph[v] contains u.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
		fn find(set: &mut [i32], x: i32) -> i32 {
			if x != set[x as usize] {
				set[x as usize] = find(set, set[x as usize]);
			}
			set[x as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}
		let mut set: Vec<i32> = (0..graph.len() as i32).collect();
		for (u, adj) in (0..).zip(graph) {
			for &v in &adj {
				if find(&mut set, u) == find(&mut set, v) {
					return false;
				}
				union(&mut set, adj[0], v);
			}
		}
		true
	}
	pub fn is_bipartite1(graph: Vec<Vec<i32>>) -> bool {
		let mut color = vec![-1; graph.len()];
		for i in 0..graph.len() {
			if color[i] < 0 && !Self::coloring(&mut color, &graph, i as i32, 0) {
				return false;
			}
		}
		true
	}

	fn coloring(color: &mut [i8], graph: &[Vec<i32>], i: i32, c: i8) -> bool {
		color[i as usize] = c;
		for &nei in &graph[i as usize] {
			match color[nei as usize] {
				-1 => {
					if !Self::coloring(color, graph, nei, c ^ 1) {
						return false;
					}
				}
				cc if cc == c => return false,
				_ => {}
			}
		}
		true
	}
}

// submission codes end
