/**
 * [407] Trapping Rain Water II
 *
 * Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap1-3d.jpg" style="width: 361px; height: 321px;" />
 * Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
 * Output: 4
 * Explanation: After the rain, water is trapped between the blocks.
 * We have two small pounds 1 and 3 units trapped.
 * The total volume of water trapped is 4.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap2-3d.jpg" style="width: 401px; height: 321px;" />
 * Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
 * Output: 10
 *
 *  
 * Constraints:
 *
 *     m == heightMap.length
 *     n == heightMap[i].length
 *     1 <= m, n <= 200
 *     0 <= heightMap[i][j] <= 2 * 10^4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn trap_rain_water(height: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (height.len(), height[0].len());
		let mut visited = vec![vec![false; n]; m];
		use std::{cmp::Reverse, collections::BinaryHeap};

		let mut bh = BinaryHeap::new();
		for (i, col) in height.iter().enumerate() {
			visited[i][0] = true;
			visited[i][n - 1] = true;
			bh.push(Reverse((col[0], i, 0)));
			bh.push(Reverse((col[n - 1], i, n - 1)));
		}
		for (j, (&r1, &r2)) in height[0].iter().zip(&height[m - 1]).enumerate() {
			visited[0][j] = true;
			visited[m - 1][j] = true;
			bh.push(Reverse((r1, 0, j)));
			bh.push(Reverse((r2, m - 1, j)));
		}

		let mut ans = 0;
		while let Some(Reverse((h, i, j))) = bh.pop() {
			//const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
			//(i, j) 全部+1免得有一大堆 as
			const DIRS: [(usize, usize); 4] = [(1, 2), (1, 0), (2, 1), (0, 1)];
			for (i, j) in DIRS.iter().map(|(a, b)| (i + a, j + b)) {
				if (1..=m).contains(&i) && (1..=n).contains(&j) && !visited[i - 1][j - 1] {
					let (i, j) = (i - 1, j - 1);
					visited[i][j] = true;
					ans += 0.max(h - height[i][j]);
					bh.push(Reverse((h.max(height[i][j]), i, j)));
				}
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_407() {
		assert_eq!(
			Solution::trap_rain_water(matrix![
				[1, 4, 3, 1, 3, 2],
				[3, 2, 1, 3, 2, 4],
				[2, 3, 3, 2, 3, 1]
			]),
			4
		);
		assert_eq!(
			Solution::trap_rain_water(matrix![
				[3, 3, 3, 3, 3],
				[3, 2, 2, 2, 3],
				[3, 2, 1, 2, 3],
				[3, 2, 2, 2, 3],
				[3, 3, 3, 3, 3]
			]),
			10
		);
		assert_eq!(
			Solution::trap_rain_water(matrix![
				[3, 3, 3, 3, 3],
				[3, 2, 2, 2, 3],
				[3, 2, 1, 1, 3],
				[3, 2, 2, 2, 3],
				[3, 3, 2, 3, 3]
			]),
			2
		);
	}
}
