/**
 * [149] Max Points on a Line
 *
 * Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 3
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 *     1 <= points.length <= 300
 *     points[i].length == 2
 *     -10^4 <= xi, yi <= 10^4
 *     All the points are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
		fn gcd(x: i32, y: i32) -> i32 {
			match y {
				0 => x.abs(),
				_ => gcd(y, x % y),
			}
		}
		// given two points, return a ***unique*** line representation;
		fn line(a: (i32, i32), b: (i32, i32)) -> (u32, i32, i32) {
			let xd = b.0 - a.0;
			let yd = b.1 - a.1;
			let c = a.0 * b.1 - a.1 * b.0;
			let div = match xd.signum() {
				0 => yd.signum(),
				signum => signum,
			} * gcd(gcd(xd, yd), c);
			((xd / div) as _, yd / div, c / div)
		}
		let points = points.into_iter().map(|p| (p[0], p[1])).collect::<Vec<_>>();
		let mut cnts = std::collections::HashMap::new();
		let mut ans = 1;
		let mut it = points.iter();
		while let Some(&a) = it.next() {
			for &b in it.clone() {
				*cnts.entry(line(a, b)).or_insert(1) += 1;
			}
			ans = cnts.drain().fold(ans, |acc, (_, v)| v.max(acc));
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_149() {
		assert_eq!(Solution::max_points(matrix![[1, 1], [2, 2], [3, 3]]), 3);
		assert_eq!(
			Solution::max_points(matrix![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]),
			4
		);
	}
}
