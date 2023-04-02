/**
 * [1466] Reorder Routes to Make All Paths Lead to the City Zero
 *
 * There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
 * Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.
 * This year, there will be a big event in the capital (city 0), and many people want to travel to this city.
 * Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.
 * It's guaranteed that each city can reach city 0 after reorder.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/13/sample_1_1819.png" style="width: 311px; height: 189px;" />
 * Input: n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
 * Output: 3
 * Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/13/sample_2_1819.png" style="width: 509px; height: 79px;" />
 * Input: n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
 * Output: 2
 * Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 3, connections = [[1,0],[2,0]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     2 <= n <= 5 * 10^4
 *     connections.length == n - 1
 *     connections[i].length == 2
 *     0 <= ai, bi <= n - 1
 *     ai != bi
 *
 */
pub struct Solution {}

impl Solution {
	pub fn min_reorder(n: i32, roads: Vec<Vec<i32>>) -> i32 {
		let mut neigs = vec![vec![]; n as usize];
		for r in &roads {
			neigs[r[0] as usize].push(-r[1]);
			neigs[r[1] as usize].push(r[0]);
		}
		Self::dfs(&neigs, 0, 0)
	}

	fn dfs(roads: &[Vec<i32>], cur: i32, from: i32) -> i32 {
		roads[cur as usize]
			.iter()
			.filter(|nei| nei.abs() != from)
			.map(|nei| nei.is_negative() as i32 + Self::dfs(roads, nei.abs(), cur))
			.sum()
	}
}
