/**
 * [1514] Path with Maximum Probability
 *
 * You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].
 * Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.
 * If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex1.png" style="width: 187px; height: 186px;" />
 *
 * Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
 * Output: 0.25000
 * Explanation: There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex2.png" style="width: 189px; height: 186px;" />
 *
 * Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
 * Output: 0.30000
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex3.png" style="width: 215px; height: 191px;" />
 *
 * Input: n = 3, edges = [[0,1]], succProb = [0.5], start = 0, end = 2
 * Output: 0.00000
 * Explanation: There is no path between 0 and 2.
 *
 *  
 * Constraints:
 *
 *     2 <= n <= 10^4
 *     0 <= start, end < n
 *     start != end
 *     0 <= a, b < n
 *     a != b
 *     0 <= succProb.length == edges.length <= 2*10^4
 *     0 <= succProb[i] <= 1
 *     There is at most one edge between every two nodes.
 *
 */
pub struct Solution {}

// submission codes start here

//TODO: optimise
impl Solution {
	pub fn max_probability(
		n: i32,
		edges: Vec<Vec<i32>>,
		succ_prob: Vec<f64>,
		start: i32,
		end: i32,
	) -> f64 {
		let mut graph = vec![vec![]; n as usize];
		for (e, p) in edges.into_iter().zip(succ_prob) {
			graph[e[0] as usize].push((e[1], p));
			graph[e[1] as usize].push((e[0], p));
		}

		let mut probs = vec![0.0; n as usize];
		Self::dijksta(&graph, start, &mut probs);
		probs[end as usize]
	}
	fn dijksta(graph: &[Vec<(i32, f64)>], s: i32, probs: &mut [f64]) {
		let mut bh = std::collections::BinaryHeap::new();
		probs[s as usize] = 1.0;
		for &(nei, p) in &graph[s as usize] {
			bh.push(((p * 1e5) as i32, s, nei));
		}
		while let Some((p, f, t)) = bh.pop() {
			if probs[f as usize] * p as f64 / 1e5 > probs[t as usize] {
				probs[t as usize] = probs[f as usize] * p as f64 / 1e5;
				for &(nei, p) in &graph[t as usize] {
					bh.push(((p * 1e5) as i32, t, nei));
				}
			}
		}
	}
}

// submission codes end
