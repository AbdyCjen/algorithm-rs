/**
 * [2352] Equal Row and Column Pairs
 *
 * Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.
 * A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/01/ex1.jpg" style="width: 150px; height: 153px;" />
 * Input: grid = [[3,2,1],[1,7,6],[2,7,7]]
 * Output: 1
 * Explanation: There is 1 equal row and column pair:
 * - (Row 2, Column 1): [2,7,7]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/01/ex2.jpg" style="width: 200px; height: 209px;" />
 * Input: grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
 * Output: 3
 * Explanation: There are 3 equal row and column pairs:
 * - (Row 0, Column 0): [3,1,2,2]
 * - (Row 2, Column 2): [2,4,2,2]
 * - (Row 3, Column 2): [2,4,2,2]
 *
 *  
 * Constraints:
 *
 *     n == grid.length == grid[i].length
 *     1 <= n <= 200
 *     1 <= grid[i][j] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
		let mut cnts = std::collections::HashMap::new();
		for row in &grid {
			*cnts.entry(&row[..]).or_insert(0) += 1;
		}
		let (mut ans, mut col) = (0, vec![0; grid.len()]);
		for j in 0..grid.len() {
			col.iter_mut().zip(&grid).for_each(|(c, cc)| *c = cc[j]);
			ans += *cnts.get(&col[..]).unwrap_or(&0);
		}
		ans
	}
}

// submission codes end
