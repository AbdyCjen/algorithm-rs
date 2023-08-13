/**
 * [2925] Maximum Score After Applying Operations on a Tree
 *
 * There is an undirected tree with n nodes labeled from 0 to n - 1, and rooted at node 0. You are given a 2D integer array edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
 * You are also given a 0-indexed integer array values of length n, where values[i] is the value associated with the i^th node.
 * You start with a score of 0. In one operation, you can:
 *
 *     Pick any node i.
 *     Add values[i] to your score.
 *     Set values[i] to 0.
 *
 * A tree is healthy if the sum of values on the path from the root to any leaf node is different than zero.
 * Return the maximum score you can obtain after performing these operations on the tree any number of times so that it remains healthy.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/10/11/graph-13-1.png" style="width: 515px; height: 443px;" />
 * Input: edges = [[0,1],[0,2],[0,3],[2,4],[4,5]], values = [5,2,5,2,1,1]
 * Output: 11
 * Explanation: We can choose nodes 1, 2, 3, 4, and 5. The value of the root is non-zero. Hence, the sum of values on the path from the root to any leaf is different than zero. Therefore, the tree is healthy and the score is values[1] + values[2] + values[3] + values[4] + values[5] = 11.
 * It can be shown that 11 is the maximum score obtainable after any number of operations on the tree.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/10/11/graph-14-2.png" style="width: 522px; height: 245px;" />
 * Input: edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]], values = [20,10,9,7,4,3,5]
 * Output: 40
 * Explanation: We can choose nodes 0, 2, 3, and 4.
 * - The sum of values on the path from 0 to 4 is equal to 10.
 * - The sum of values on the path from 0 to 3 is equal to 10.
 * - The sum of values on the path from 0 to 5 is equal to 3.
 * - The sum of values on the path from 0 to 6 is equal to 5.
 * Therefore, the tree is healthy and the score is values[0] + values[2] + values[3] + values[4] = 40.
 * It can be shown that 40 is the maximum score obtainable after any number of operations on the tree.
 *
 *  
 * Constraints:
 *
 *     2 <= n <= 2 * 10^4
 *     edges.length == n - 1
 *     edges[i].length == 2
 *     0 <= ai, bi < n
 *     values.length == n
 *     1 <= values[i] <= 10^9
 *     The input is generated such that edges represents a valid tree.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
		let mut grid = vec![];
		for v in values {
			grid.push((v, vec![]));
		}
		for e in edges {
			grid[e[0] as usize].1.push(e[1]);
			grid[e[1] as usize].1.push(e[0]);
		}

		Self::dfs(0, &grid, -1).0
	}

	fn dfs(cur: usize, grid: &[(i32, Vec<i32>)], prv: i32) -> (i64, i64) {
		let mut s = 0;
		let mut a = 0;
		let mut cnt = 0;
		for &nei in &grid[cur].1 {
			if nei != prv {
				cnt += 1;
				let (ca, cs) = Self::dfs(nei as usize, grid, cur as i32);
				a += ca;
				s += cs;
			}
		}

		if cnt > 0 {
			((a + grid[cur].0 as i64).max(s), s + grid[cur].0 as i64)
		} else {
			// leaf
			(a, s + grid[cur].0 as i64)
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2925() {
		assert_eq!(
			Solution::maximum_score_after_operations(matrix![[0, 1]], vec![1, 2],),
			2
		);
		assert_eq!(
			Solution::maximum_score_after_operations(
				matrix![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
				vec![20, 10, 9, 7, 4, 3, 5]
			),
			40
		);
	}
}
