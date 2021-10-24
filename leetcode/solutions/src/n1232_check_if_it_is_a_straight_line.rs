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

#[allow(dead_code)]
impl Solution {
	pub fn check_straight_line(points: Vec<Vec<i32>>) -> bool {
		if points.len() <= 2 {
			return true;
		}
		let (x1, y1) = (points[0][0], points[0][1]);
		let (x2, y2) = (points[1][0], points[1][1]);
		points
			.into_iter()
			.skip(2)
			.all(|p| (p[1] - y1) * (p[0] - x2) == (p[1] - y2) * (p[0] - x1))
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1349() {
		assert!(Solution::check_straight_line(vec![
			vec![1, 2],
			vec![2, 3],
			vec![3, 4],
			vec![4, 5],
			vec![5, 6],
			vec![6, 7]
		]));
		assert!(!Solution::check_straight_line(vec![
			vec![1, 1],
			vec![2, 2],
			vec![3, 4],
			vec![4, 5],
			vec![5, 6],
			vec![7, 7]
		]));
		assert!(Solution::check_straight_line(vec![
			vec![1, 1],
			vec![1, 1],
			vec![1, 1]
		]));
	}
}
