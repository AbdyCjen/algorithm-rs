/**
 * [223] Rectangle Area
 *
 * Given the coordinates of two rectilinear rectangles in a 2D plane, return the total area covered by the two rectangles.
 * The first rectangle is defined by its bottom-left corner (ax1, ay1) and its top-right corner (ax2, ay2).
 * The second rectangle is defined by its bottom-left corner (bx1, by1) and its top-right corner (bx2, by2).
 *  
 * <strong class="example">Example 1:
 * <img alt="Rectangle Area" src="https://assets.leetcode.com/uploads/2021/05/08/rectangle-plane.png" style="width: 700px; height: 365px;" />
 * Input: ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9, by2 = 2
 * Output: 45
 *
 * <strong class="example">Example 2:
 *
 * Input: ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2, by2 = 2
 * Output: 16
 *
 *  
 * Constraints:
 *
 *     -10^4 <= ax1 <= ax2 <= 10^4
 *     -10^4 <= ay1 <= ay2 <= 10^4
 *     -10^4 <= bx1 <= bx2 <= 10^4
 *     -10^4 <= by1 <= by2 <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(clippy::too_many_arguments)]
impl Solution {
	pub fn compute_area(
		ax1: i32,
		ay1: i32,
		ax2: i32,
		ay2: i32,
		bx1: i32,
		by1: i32,
		bx2: i32,
		by2: i32,
	) -> i32 {
		let x1 = bx1.clamp(ax1, ax2);
		let x2 = bx2.clamp(ax1, ax2);
		let y1 = by1.clamp(ay1, ay2);
		let y2 = by2.clamp(ay1, ay2);

		(ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - (x2 - x1) * (y2 - y1)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_223() {
		assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
		assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
	}
}
