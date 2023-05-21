/**
 * [1857] Largest Color Value in a Directed Graph
 *
 * Thre is a directed graph of n colored nodes and m edges. The nodes are numbered from 0 to n - 1.
 *
 * You are given a string colors where colors[i] is a lowercase English letter representing the color of the i^th node in this graph (0-indexed). You are also given a 2D array edges where edges[j] = [aj, bj] indicates that there is a directed edge from node aj to node bj.
 *
 * A valid path in the graph is a sequence of nodes x1 -> x2 -> x3 -> ... -> xk such that there is a directed edge from xi to xi+1 for every 1 <= i < k. The color value of the path is the number of nodes that are colored the most frequently occurring color along that path.
 *
 * Return the largest color value of any valid path in the given graph, or -1 if the graph contains a cycle.
 *
 *  
 * <strong class="example">Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/21/leet1.png" style="width: 400px; height: 182px;" />
 *
 *
 * Input: colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
 * Output: 3
 * Explanation: The path 0 -> 2 -> 3 -> 4 contains 3 nodes that are colored "a" (red in the above image).
 *
 *
 * <strong class="example">Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/21/leet2.png" style="width: 85px; height: 85px;" />
 *
 *
 * Input: colors = "a", edges = [[0,0]]
 * Output: -1
 * Explanation: There is a cycle from 0 to 0.
 *
 *
 *  
 * Constraints:
 *
 *
 *     n == colors.length
 *     m == edges.length
 *     1 <= n <= 10^5
 *     0 <= m <= 10^5
 *     colors consists of lowercase English letters.
 *     0 <= aj, bj < n
 *
 */
pub struct Solution {}

impl Solution {
	pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
		let colors = colors.into_bytes();
		let mut neigs = vec![(vec![], 0, [0; 26]); colors.len()];
		for e in edges {
			neigs[e[0] as usize].0.push(e[1]);
			neigs[e[1] as usize].1 += 1;
		}
		let mut st: Vec<i32> = neigs
			.iter()
			.zip(0..)
			.filter(|v| v.0 .1 == 0)
			.map(|(_, i)| i)
			.collect();
		let mut cnt = 0;
		let mut ans = 0;
		while let Some(cur) = st.pop() {
			cnt += 1;
			let mut ccnt = neigs[cur as usize].2;
			ccnt[(colors[cur as usize] - b'a') as usize] += 1;
			ans = ans.max(ccnt[(colors[cur as usize] - b'a') as usize]);
			for nei in std::mem::take(&mut neigs[cur as usize].0) {
				for (c1, c2) in neigs[nei as usize].2.iter_mut().zip(ccnt) {
					*c1 = c2.max(*c1);
				}
				neigs[nei as usize].1 -= 1;
				if neigs[nei as usize].1 == 0 {
					st.push(nei);
				}
			}
		}
		if cnt == colors.len() {
			ans
		} else {
			-1
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1857() {
		assert_eq!(
			Solution::largest_path_value(
				"abaca".to_owned(),
				matrix![[0, 1], [0, 2], [2, 3], [3, 4]]
			),
			3
		);
		assert_eq!(
			Solution::largest_path_value("a".to_owned(), matrix![[0, 0]]),
			-1
		);
	}
}
