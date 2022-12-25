/**
 * [886] Possible Bipartition
 *
 * We want to split a group of n people (labeled from 1 to n) into two groups of any size. Each person may dislike some other people, and they should not go into the same group.
 * Given the integer n and the array dislikes where dislikes[i] = [ai, bi] indicates that the person labeled ai does not like the person labeled bi, return true if it is possible to split everyone into two groups in this way.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 4, dislikes = [[1,2],[1,3],[2,4]]
 * Output: true
 * Explanation: group1 [1,4] and group2 [2,3].
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 3, dislikes = [[1,2],[1,3],[2,3]]
 * Output: false
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 5, dislikes = [[1,2],[2,3],[3,4],[4,5],[1,5]]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 2000
 *     0 <= dislikes.length <= 10^4
 *     dislikes[i].length == 2
 *     1 <= dislikes[i][j] <= n
 *     ai < bi
 *     All the pairs of dislikes are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
		let mut neigh = vec![vec![]; n as usize + 1];
		for dis in dislikes {
			neigh[dis[0] as usize].push(dis[1]);
			neigh[dis[1] as usize].push(dis[0]);
		}
		let mut color = vec![0; n as usize + 1];

		for i in 1..=n {
			if color[i as usize] == 0 && !Self::dfs(&mut color, &neigh, i, 1) {
				return false;
			}
		}

		true
	}

	fn dfs(color: &mut [i32], neigh: &[Vec<i32>], i: i32, c: i32) -> bool {
		color[i as usize] = c;
		for &nei in &neigh[i as usize] {
			if color[nei as usize] == 0 && !Self::dfs(color, neigh, nei, -c)
				|| color[nei as usize] == c
			{
				return false;
			}
		}
		true
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_886() {}
}
