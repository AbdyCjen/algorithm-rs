/**
 * [118] Pascal's Triangle
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *  
 * <strong class="example">Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * <strong class="example">Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 *  
 * Constraints:
 *
 *     1 <= numRows <= 30
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
		let mut ans = vec![vec![1]];
		for i in 1..num_rows as usize {
			ans.push(vec![1; i + 1]);
			for j in 1..i {
				ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
			}
		}
		ans
	}
}

// submission codes end
