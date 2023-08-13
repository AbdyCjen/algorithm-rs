/**
 * [1349] Check If It Is a Straight Line
 *
 * You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.
 *  
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/15/untitled-diagram-2.jpg" style="width: 336px; height: 336px;" />
 *
 * Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/09/untitled-diagram-1.jpg" style="width: 348px; height: 336px;" />
 *
 * Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 2 <= coordinates.length <= 1000
 * coordinates[i].length == 2
 * -10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
 * coordinates contains no duplicate point.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn check_straight_line(points: Vec<Vec<i32>>) -> bool {
		match &points[..] {
			[p1, p2, points @ ..] => points
				.iter()
				.all(|p| (p[1] - p1[1]) * (p[0] - p2[0]) == (p[1] - p2[1]) * (p[0] - p1[0])),
			_ => true,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1349() {
		assert!(Solution::check_straight_line(matrix![
			[1, 2],
			[2, 3],
			[3, 4],
			[4, 5],
			[5, 6],
			[6, 7]
		]));
		assert!(!Solution::check_straight_line(matrix![
			[1, 1],
			[2, 2],
			[3, 4],
			[4, 5],
			[5, 6],
			[7, 7]
		]));
		assert!(Solution::check_straight_line(matrix![
			[1, 1],
			[1, 1],
			[1, 1]
		]));
	}
}
