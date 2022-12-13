/**
 * [2500] Delete Greatest Value in Each Row
 *
 * You are given an m x n matrix grid consisting of positive integers.
 * Perform the following operation until grid becomes empty:
 *
 *     Delete the element with the greatest value from each row. If multiple such elements exist, delete any of them.
 *     Add the maximum of deleted elements to the answer.
 *
 * Note that the number of columns decreases by one after each operation.
 * Return the answer after performing the operations described above.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/19/q1ex1.jpg" style="width: 600px; height: 135px;" />
 * Input: grid = [[1,2,4],[3,3,1]]
 * Output: 8
 * Explanation: The diagram above shows the removed values in each step.
 * - In the first operation, we remove 4 from the first row and 3 from the second row (notice that, there are two cells with value 3 and we can remove any of them). We add 4 to the answer.
 * - In the second operation, we remove 2 from the first row and 3 from the second row. We add 3 to the answer.
 * - In the third operation, we remove 1 from the first row and 1 from the second row. We add 1 to the answer.
 * The final answer = 4 + 3 + 1 = 8.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/19/q1ex2.jpg" style="width: 83px; height: 83px;" />
 * Input: grid = [[10]]
 * Output: 10
 * Explanation: The diagram above shows the removed values in each step.
 * - In the first operation, we remove 10 from the first row. We add 10 to the answer.
 * The final answer = 10.
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 50
 *     1 <= grid[i][j] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
		grid.iter_mut().for_each(|v| v.sort_unstable());
		let mut ans = 0;
		while !grid[0].is_empty() {
			let mut max = 0;
			for row in &mut grid {
				if let Some(n) = row.pop() {
					max = max.max(n);
				}
			}
			ans += max;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2500() {
		assert_eq!(
			Solution::delete_greatest_value(matrix![[1, 2, 4], [3, 3, 1]]),
			8
		);
		assert_eq!(Solution::delete_greatest_value(matrix![[10]]), 10);
	}
}
