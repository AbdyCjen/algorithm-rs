/**
 * [836] Rectangle Overlap
 *
 * An axis-aligned rectangle is represented as a list [x1, y1, x2, y2], where (x1, y1) is the coordinate of its bottom-left corner, and (x2, y2) is the coordinate of its top-right corner. Its top and bottom edges are parallel to the X-axis, and its left and right edges are parallel to the Y-axis.
 * Two rectangles overlap if the area of their intersection is positive. To be clear, two rectangles that only touch at the corner or edges do not overlap.
 * Given two axis-aligned rectangles rec1 and rec2, return true if they overlap, otherwise return false.
 *  
 * Example 1:
 * Input: rec1 = [0,0,2,2], rec2 = [1,1,3,3]
 * Output: true
 * Example 2:
 * Input: rec1 = [0,0,1,1], rec2 = [1,0,2,1]
 * Output: false
 * Example 3:
 * Input: rec1 = [0,0,1,1], rec2 = [2,2,3,3]
 * Output: false
 *  
 * Constraints:
 *
 *     rect1.length == 4
 *     rect2.length == 4
 *     -10^9 <= rec1[i], rec2[i] <= 10^9
 *     rec1 and rec2 represent a valid rectangle with a non-zero area.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn is_rectangle_overlap(r1: Vec<i32>, r2: Vec<i32>) -> bool {
		r1[0] < r2[2] && r1[2] > r2[0] && r1[1] < r2[3] && r1[3] > r2[1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_836() {
		assert!(Solution::is_rectangle_overlap(
			vec![0, 0, 2, 2],
			vec![1, 1, 3, 3]
		));
		assert!(!Solution::is_rectangle_overlap(
			vec![0, 0, 1, 1],
			vec![1, 0, 2, 1]
		));
		assert!(!Solution::is_rectangle_overlap(
			vec![0, 0, 1, 1],
			vec![2, 2, 3, 3]
		));
	}
}
