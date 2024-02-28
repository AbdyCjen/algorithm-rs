/**
 * [1463] Cherry Pickup II
 *
 * You are given a rows x cols matrix grid representing a field of cherries where grid[i][j] represents the number of cherries that you can collect from the (i, j) cell.
 * You have two robots that can collect cherries for you:
 *
 *     Robot #1 is located at the top-left corner (0, 0), and
 *     Robot #2 is located at the top-right corner (0, cols - 1).
 *
 * Return the maximum number of cherries collection using both robots by following the rules below:
 *
 *     From a cell (i, j), robots can move to cell (i + 1, j - 1), (i + 1, j), or (i + 1, j + 1).
 *     When any robot passes through a cell, It picks up all cherries, and the cell becomes an empty cell.
 *     When both robots stay in the same cell, only one takes the cherries.
 *     Both robots cannot move outside of the grid at any moment.
 *     Both robots should reach the bottom row in grid.
 *
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/29/sample_1_1802.png" style="width: 374px; height: 501px;" />
 * Input: grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]
 * Output: 24
 * Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
 * Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
 * Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
 * Total of cherries: 12 + 12 = 24.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/sample_2_1802.png" style="width: 500px; height: 452px;" />
 * Input: grid = [[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]
 * Output: 28
 * Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
 * Cherries taken by Robot #1, (1 + 9 + 5 + 2) = 17.
 * Cherries taken by Robot #2, (1 + 3 + 4 + 3) = 11.
 * Total of cherries: 17 + 11 = 28.
 *
 *  
 * Constraints:
 *
 *     rows == grid.length
 *     cols == grid[i].length
 *     2 <= rows, cols <= 70
 *     0 <= grid[i][j] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
		let n = grid[0].len();
		let mut dp = vec![vec![0; n]; n];
		let mut new = dp.clone();
		dp[0][n - 1] = grid[0][0] + grid[0][n - 1];
		for (r, row) in (1..).zip(grid.into_iter().skip(1)) {
			for i in 0..r.min(n) {
				for j in (i + 1).max(n.saturating_sub(r))..n {
					for ii in i.saturating_sub(1)..n.min(i + 2) {
						for jj in j.saturating_sub(1)..n.min(j + 2) {
							if ii == jj {
								new[ii][jj] = new[ii][jj].max(dp[i][j] + row[ii]);
							} else {
								new[ii][jj] = new[ii][jj].max(dp[i][j] + row[ii] + row[jj]);
							}
						}
					}
				}
			}
			std::mem::swap(&mut new, &mut dp);
		}
		dp.into_iter().flatten().max().unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1463() {
		assert_eq!(
			Solution::cherry_pickup(matrix![[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]]),
			24
		);
		assert_eq!(
			Solution::cherry_pickup(matrix![
				[0, 8, 7, 10, 9, 10, 0, 9, 6],
				[8, 7, 10, 8, 7, 4, 9, 6, 10],
				[8, 1, 1, 5, 1, 5, 5, 1, 2],
				[9, 4, 10, 8, 8, 1, 9, 5, 0],
				[4, 3, 6, 10, 9, 2, 4, 8, 10],
				[7, 3, 2, 8, 3, 3, 5, 9, 8],
				[1, 2, 6, 5, 6, 2, 0, 10, 0]
			]),
			96
		);
		assert_eq!(
			Solution::cherry_pickup(matrix![
				[1, 0, 0, 0, 0, 0, 1],
				[2, 0, 0, 0, 0, 3, 0],
				[2, 0, 9, 0, 0, 0, 0],
				[0, 3, 0, 5, 4, 0, 0],
				[1, 0, 2, 3, 0, 0, 6]
			]),
			28
		);
	}
}
