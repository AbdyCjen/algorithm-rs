/**
 * [2642] Design Graph With Shortest Path Calculator
 *
 * There is a directed weighted graph that consists of n nodes numbered from 0 to n - 1. The edges of the graph are initially represented by the given array edges where edges[i] = [fromi, toi, edgeCosti] meaning that there is an edge from fromi to toi with the cost edgeCosti.
 * Implement the Graph class:
 *
 *     Graph(int n, int[][] edges) initializes the object with n nodes and the given edges.
 *     addEdge(int[] edge) adds an edge to the list of edges where edge = [from, to, edgeCost]. It is guaranteed that there is no edge between the two nodes before adding this one.
 *     int shortestPath(int node1, int node2) returns the minimum cost of a path from node1 to node2. If no path exists, return -1. The cost of a path is the sum of the costs of the edges in the path.
 *
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/01/11/graph3drawio-2.png" style="width: 621px; height: 191px;" />
 * Input
 * ["Graph", "shortestPath", "shortestPath", "addEdge", "shortestPath"]
 * [[4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]], [3, 2], [0, 3], [[1, 3, 4]], [0, 3]]
 * Output
 * [null, 6, -1, null, 6]
 * Explanation
 * Graph g = new Graph(4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
 * g.shortestPath(3, 2); // return 6. The shortest path from 3 to 2 in the first diagram above is 3 -> 0 -> 1 -> 2 with a total cost of 3 + 2 + 1 = 6.
 * g.shortestPath(0, 3); // return -1. There is no path from 0 to 3.
 * g.addEdge([1, 3, 4]); // We add an edge from node 1 to node 3, and we get the second diagram above.
 * g.shortestPath(0, 3); // return 6. The shortest path from 0 to 3 now is 0 -> 1 -> 3 with a total cost of 2 + 4 = 6.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 100
 *     0 <= edges.length <= n * (n - 1)
 *     edges[i].length == edge.length == 3
 *     0 <= fromi, toi, from, to, node1, node2 <= n - 1
 *     1 <= edgeCosti, edgeCost <= 10^6
 *     There are no repeated edges and no self-loops in the graph at any point.
 *     At most 100 calls will be made for addEdge.
 *     At most 100 calls will be made for shortestPath.
 *
 */
// submission codes start here
struct Graph(Vec<Vec<(i32, i32)>>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// TODO: floyd
impl Graph {
	fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
		let mut graph = Self(vec![Default::default(); n as usize]);
		for e in edges {
			graph.0[e[0] as usize].push((e[1], e[2]));
		}
		graph
	}

	fn add_edge(&mut self, e: Vec<i32>) { self.0[e[0] as usize].push((e[1], e[2])); }

	fn shortest_path(&self, node1: i32, node2: i32) -> i32 { self.dfs(node1, node2).unwrap_or(-1) }

	fn dfs(&self, cur: i32, dest: i32) -> Option<i32> {
		let mut bh = std::collections::BinaryHeap::new();
		let mut dis = vec![i32::MAX; self.0.len()];
		dis[cur as usize] = 0;
		for &(nei, d) in &self.0[cur as usize] {
			bh.push((-d, cur, nei));
		}
		while let Some((nw, f, t)) = bh.pop() {
			if dis[f as usize] - nw < dis[t as usize] {
				dis[t as usize] = dis[f as usize] - nw;
				for &(nei, w) in &self.0[t as usize] {
					bh.push((-w, t, nei));
				}
			}
		}
		match dis[dest as usize] {
			i32::MAX => None,
			d => Some(d),
		}
	}
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2642() {}
}
