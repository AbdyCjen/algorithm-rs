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

#[allow(dead_code)]
impl Solution {
	// 暴力三层循环运行时间反而更好
	pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
		use std::collections::HashMap;
		fn gcd(x: i32, y: i32) -> i32 {
			if y == 0 {
				x.abs()
			} else {
				gcd(y, x % y)
			}
		}
		fn line(a: (i32, i32), b: (i32, i32)) -> (u32, i32, i32) {
			let yd = b.1 - a.1;
			let xd = b.0 - a.0;
			let (a, b, c) = (xd, yd, a.1 * xd - a.0 * yd);
			let div = gcd(gcd(a, b), c);
			((a / div) as _, b / div, c / div)
		}
		let mut count: HashMap<_, (i32, i32)> = HashMap::new();
		let mut points: Vec<_> = points.into_iter().map(|p| (p[0], p[1])).collect();
		points.sort_unstable();
		for (i, a) in points.iter().enumerate() {
			for b in &points[i + 1..] {
				let cur = line(*a, *b);
				let ent = count.entry(cur).or_insert((i as _, 1));
				if ent.0 == i as i32 {
					ent.1 += 1;
				}
			}
		}
		count.into_iter().map(|e| e.1 .1).max().unwrap_or(1)
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
