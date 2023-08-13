/**
 * [1970] Last Day Where You Can Still Cross
 *
 * There is a 1-based binary matrix where 0 represents land and 1 represents water. You are given integers row and col representing the number of rows and columns in the matrix, respectively.
 * Initially on day 0, the entire matrix is land. However, each day a new cell becomes flooded with water. You are given a 1-based 2D array cells, where cells[i] = [ri, ci] represents that on the i^th day, the cell on the ri^th row and ci^th column (1-based coordinates) will be covered with water (i.e., changed to 1).
 * You want to find the last day that it is possible to walk from the top to the bottom by only walking on land cells. You can start from any cell in the top row and end at any cell in the bottom row. You can only travel in the four cardinal directions (left, right, up, and down).
 * Return the last day where it is possible to walk from the top to the bottom by only walking on land cells.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/27/1.png" style="width: 624px; height: 162px;" />
 * Input: row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
 * Output: 2
 * Explanation: The above image depicts how the matrix changes each day starting from day 0.
 * The last day where it is possible to cross from top to bottom is on day 2.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/27/2.png" style="width: 504px; height: 178px;" />
 * Input: row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
 * Output: 1
 * Explanation: The above image depicts how the matrix changes each day starting from day 0.
 * The last day where it is possible to cross from top to bottom is on day 1.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/27/3.png" style="width: 666px; height: 167px;" />
 * Input: row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
 * Output: 3
 * Explanation: The above image depicts how the matrix changes each day starting from day 0.
 * The last day where it is possible to cross from top to bottom is on day 3.
 *
 *  
 * Constraints:
 *
 *     2 <= row, col <= 2 * 10^4
 *     4 <= row * col <= 2 * 10^4
 *     cells.length == row * col
 *     1 <= ri <= row
 *     1 <= ci <= col
 *     All the values of cells are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn latest_day_to_cross(m: i32, n: i32, cells: Vec<Vec<i32>>) -> i32 {
		let mut mat = vec![vec![0; n as usize]; m as usize];
		for (n, pos) in (1..).zip(cells) {
			mat[pos[0] as usize - 1][pos[1] as usize - 1] = n;
		}
		let (mut l, mut r) = (n - 1, m * n);
		while l <= r {
			let mid = (l + r) / 2;
			if Self::cross(&mat, mid) {
				l = mid + 1;
			} else {
				r = mid - 1;
			}
		}
		r
	}

	fn cross(mat: &[Vec<i32>], k: i32) -> bool {
		let (m, n) = (mat.len() as i32, mat[0].len() as i32);
		let mut st: Vec<_> = (0..)
			.zip(&mat[0])
			.filter(|(_, c)| **c > k)
			.map(|(j, _)| (0, j))
			.collect();
		let mut mat = mat.to_vec();
		mat[0].iter_mut().for_each(|c| *c = 0);
		while let Some((i, j)) = st.pop() {
			if i == m - 1 {
				return true;
			}
			for (ni, nj) in [(i + 1, j), (i - 1, j), (i, j - 1), (i, j + 1)] {
				if ni >= 0 && nj >= 0 && ni < m && nj < n && mat[ni as usize][nj as usize] > k {
					mat[ni as usize][nj as usize] = 0;
					st.push((ni, nj));
				}
			}
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1970() {
		assert_eq!(
			Solution::latest_day_to_cross(
				6,
				2,
				matrix![
					[4, 2],
					[6, 2],
					[2, 1],
					[4, 1],
					[6, 1],
					[3, 1],
					[2, 2],
					[3, 2],
					[1, 1],
					[5, 1],
					[5, 2],
					[1, 2]
				]
			),
			3
		);
		assert_eq!(
			Solution::latest_day_to_cross(
				3,
				3,
				matrix![
					[1, 2],
					[2, 1],
					[3, 3],
					[2, 2],
					[1, 1],
					[1, 3],
					[2, 3],
					[3, 2],
					[3, 1]
				]
			),
			3
		);
		assert_eq!(
			Solution::latest_day_to_cross(2, 2, matrix![[1, 1], [2, 1], [1, 2], [2, 2]]),
			2
		);
	}
}
