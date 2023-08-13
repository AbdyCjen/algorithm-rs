/**
 * [1351] Count Negative Numbers in a Sorted Matrix
 *
 * Given a m x n matrix grid which is sorted in non-increasing order both row-wise and column-wise, return the number of negative numbers in grid.
 *  
 * <strong class="example">Example 1:
 *
 * Input: grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
 * Output: 8
 * Explanation: There are 8 negatives number in the matrix.
 *
 * <strong class="example">Example 2:
 *
 * Input: grid = [[3,2],[1,0]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 100
 *     -100 <= grid[i][j] <= 100
 *
 *  
 * Follow up: Could you find an O(n + m) solution?
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
		let (mut ans, n) = (0, grid[0].len());
		let mut i = n;
		for row in grid {
			while i > 0 && row[i - 1] < 0 {
				i -= 1;
			}
			ans += (n - i) as i32;
		}
		ans
	}
	pub fn count_negatives1(grid: Vec<Vec<i32>>) -> i32 {
		grid.into_iter().flatten().filter(|&i| i < 0).count() as i32
	}
}

// submission codes end
