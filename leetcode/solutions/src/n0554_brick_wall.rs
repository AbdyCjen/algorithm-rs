/**
 * [554] Brick Wall
 *
 * There is a brick wall in front of you. The wall is rectangular and has several rows of bricks. The bricks have the same height but different width. You want to draw a vertical line from the top to the bottom and cross the least bricks.
 *
 * The brick wall is represented by a list of rows. Each row is a list of integers representing the width of each brick in this row from left to right.
 *
 * If your line go through the edge of a brick, then the brick is not considered as crossed. You need to find out how to draw the line to cross the least bricks and return the number of crossed bricks.
 *
 * You cannot draw a line just along one of the two vertical edges of the wall, in which case the line will obviously cross no bricks.
 *
 *  
 *
 * Example:
 *
 *
 * Input: [[1,2,2,1],
 *         [3,1,2],
 *         [1,3,2],
 *         [2,4],
 *         [3,1,2],
 *         [1,3,1,1]]
 *
 * Output: 2
 *
 * Explanation:
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/brick_wall.png" style="width: 100%; max-width: 350px" />
 *
 *
 *  
 *
 * Note:
 *
 * <ol>
 * The width sum of bricks in different rows are the same and won't exceed INT_MAX.
 * The number of bricks in each row is in range [1,10,000]. The height of wall is in range [1,10,000]. Total number of bricks of the wall won't exceed 20,000.
 * </ol>
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
	pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
		let mut cmap = HashMap::<i32, i32>::new();
		let h: i32 = wall.len() as i32;
		wall.into_iter().for_each(|v| {
			v.into_iter().fold(0, |sum, i| {
				*cmap.entry(sum).or_insert(0) += 1;
				sum + i
			});
		});
		*cmap.entry(0).or_insert(0) = 0;
		h - cmap
			.into_iter()
			.fold(0, |sum, (_, v)| std::cmp::max(sum, v))
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_554() {
		assert_eq!(
			Solution::least_bricks(vec![
				vec![1, 2, 2, 1],
				vec![3, 1, 2],
				vec![1, 3, 2],
				vec![2, 4],
				vec![3, 1, 2],
				vec![1, 3, 1, 1]
			]),
			2
		);
	}
}
