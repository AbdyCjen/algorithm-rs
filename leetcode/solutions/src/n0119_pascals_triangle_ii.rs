/**
 * [119] Pascal's Triangle II
 *
 * Given an integer rowIndex, return the rowIndex^th (0-indexed) row of the Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 * <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height:240px; width:260px" />
 *  
 * <strong class="example">Example 1:
 * Input: rowIndex = 3
 * Output: [1,3,3,1]
 * <strong class="example">Example 2:
 * Input: rowIndex = 0
 * Output: [1]
 * <strong class="example">Example 3:
 * Input: rowIndex = 1
 * Output: [1,1]
 *  
 * Constraints:
 *
 *     0 <= rowIndex <= 33
 *
 *  
 * Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn get_row(row_index: i32) -> Vec<i32> {
		let mut row = vec![1];
		for _ in 0..row_index {
			for i in (1..row.len()).rev() {
				row[i] += row[i - 1];
			}
			row.push(1);
		}
		row
	}
}

// submission codes end
