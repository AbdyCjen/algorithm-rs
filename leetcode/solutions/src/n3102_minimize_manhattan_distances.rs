/**
 * [3102] Minimize Manhattan Distances
 *
 * You are given a array points representing integer coordinates of some points on a 2D plane, where points[i] = [xi, yi].
 * The distance between two points is defined as their <span data-keyword="manhattan-distance">Manhattan distance</span>.
 * Return the minimum possible value for maximum distance between any two points by removing exactly one point.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">points = [[3,10],[5,15],[10,2],[4,4]]</span>
 * Output: <span class="example-io">12</span>
 * Explanation:
 * The maximum distance after removing each point is the following:
 *
 *     After removing the 0^th point the maximum distance is between points (5, 15) and (10, 2), which is |5 - 10| + |15 - 2| = 18.
 *     After removing the 1^st point the maximum distance is between points (3, 10) and (10, 2), which is |3 - 10| + |10 - 2| = 15.
 *     After removing the 2^nd point the maximum distance is between points (5, 15) and (4, 4), which is |5 - 4| + |15 - 4| = 12.
 *     After removing the 3^rd point the maximum distance is between points (5, 15) and (10, 2), which is |5 - 10| + |15 - 2| = 18.
 *
 * 12 is the minimum possible maximum distance between any two points after removing exactly one point.
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">points = [[1,1],[1,1],[1,1]]</span>
 * Output: <span class="example-io">0</span>
 * Explanation:
 * Removing any of the points results in the maximum distance between any two points of 0.
 * </div>
 *  
 * Constraints:
 *
 *     3 <= points.length <= 10^5
 *     points[i].length == 2
 *     1 <= points[i][0], points[i][1] <= 10^8
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
		// find the two points p1 and p2 that have the maximum distance
		let (_, p1, p2) = Self::solve(&points);
		let r = Self::solve(points[..p1].iter().chain(&points[p1 + 1..])).0;
		Self::solve(points[..p2].iter().chain(&points[p2 + 1..]))
			.0
			.min(r)
	}

	fn solve<'a>(points: impl IntoIterator<Item = &'a Vec<i32>>) -> (i32, usize, usize) {
		// for p1 = (x1, y1), p2 = (x2, y2), the Manhattan distance is |x1 - x2| + |y1 - y2|
		// can also be written as max((x1 - x2 + y1 - y2), (x2 - x1 + y1 - y2), (x1 - x2 + y2 - y1), (x2 - x1 + y2 - y1))
		//  euqal to: max((x1 + y1) - (x2 + y2), (x2 + y1) - (x1 + y2), (x1 - y1) - (x2 - y2), (x2 - y1) - (x1 - y2))
		//  for every point p = (x ,y), we can calculate the four values above and store the min and max value for each of them
		let mut min = [(i32::MAX, 0); 4];
		let mut max = [(i32::MIN, 0); 4];
		for (i, p) in (0..).zip(points) {
			for (j, p) in (0..).zip([p[0] + p[1], -p[0] - p[1], p[0] - p[1], -p[0] + p[1]]) {
				min[j] = min[j].min((p, i));
				max[j] = max[j].max((p, i));
			}
		}
		let mut ans = (0, 0, 0);
		for i in 0..4 {
			ans = ans.max((max[i].0 - min[i].0, min[i].1, max[i].1));
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_3102() {
		assert_eq!(
			Solution::minimum_distance(matrix![[3, 10], [5, 15], [10, 2], [4, 4]]),
			12
		);
	}
}
