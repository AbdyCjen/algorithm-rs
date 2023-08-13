/**
 * [802] Find Eventual Safe States
 *
 * There is a directed graph of n nodes with each node labeled from 0 to n - 1. The graph is represented by a 0-indexed 2D integer array graph where graph[i] is an integer array of nodes adjacent to node i, meaning there is an edge from node i to each node in graph[i].
 * A node is a terminal node if there are no outgoing edges. A node is a safe node if every possible path starting from that node leads to a terminal node (or another safe node).
 * Return an array containing all the safe nodes of the graph. The answer should be sorted in ascending order.
 *  
 * <strong class="example">Example 1:
 * <img alt="Illustration of graph" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/17/picture1.png" style="height: 171px; width: 600px;" />
 * Input: graph = [[1,2],[2,3],[5],[0],[5],[],[]]
 * Output: [2,4,5,6]
 * Explanation: The given graph is shown above.
 * Nodes 5 and 6 are terminal nodes as there are no outgoing edges from either of them.
 * Every path starting at nodes 2, 4, 5, and 6 all lead to either node 5 or 6.
 * <strong class="example">Example 2:
 *
 * Input: graph = [[1,2,3,4],[1,2],[3,4],[0,4],[]]
 * Output: [4]
 * Explanation:
 * Only node 4 is a terminal node, and every path starting at node 4 leads to node 4.
 *
 *  
 * Constraints:
 *
 *     n == graph.length
 *     1 <= n <= 10^4
 *     0 <= graph[i].length <= n
 *     0 <= graph[i][j] <= n - 1
 *     graph[i] is sorted in a strictly increasing order.
 *     The graph may contain self-loops.
 *     The number of edges in the graph will be in the range [1, 4 * 10^4].
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn eventual_safe_nodes(mut graph: Vec<Vec<i32>>) -> Vec<i32> {
		let mut visited = vec![false; graph.len()];
		(0..graph.len() as i32)
			.filter(|&i| Self::solve(&mut graph, &mut visited, i) == 0)
			.collect()
	}
	fn solve(graph: &mut [Vec<i32>], visited: &mut [bool], cur: i32) -> i32 {
		if !std::mem::replace(&mut visited[cur as usize], true) {
			let mut i = 0;
			while i < graph[cur as usize].len() {
				if Self::solve(graph, visited, graph[cur as usize][i]) == 0 {
					graph[cur as usize].swap_remove(i);
				} else {
					i += 1;
				}
			}
		}
		graph[cur as usize].len() as i32
	}
	pub fn eventual_safe_nodes1(graph: Vec<Vec<i32>>) -> Vec<i32> {
		let n = graph.len();
		let mut rev = vec![vec![]; n];
		let mut cnt = vec![0; n];
		let mut st = vec![];
		for (i, e) in (0..).zip(graph) {
			if e.is_empty() {
				st.push(i);
			}
			cnt[i as usize] = e.len() as i32;
			for nei in e {
				rev[nei as usize].push(i);
			}
		}

		let mut ans = vec![];
		while let Some(n) = st.pop() {
			ans.push(n);
			for &nei in &rev[n as usize] {
				cnt[nei as usize] -= 1;
				if cnt[nei as usize] == 0 {
					st.push(nei);
				}
			}
		}
		ans.sort_unstable();
		ans
	}
}

// submission codes end
