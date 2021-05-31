/**
 * [120] Triangle
 *
 * Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.
 *
 * For example, given the following triangle
 *
 *
 * [
 *      [2],
 *     [3,4],
 *    [6,5,7],
 *   [4,1,8,3]
 * ]
 *
 *
 * The minimum path sum from top to bottom is 11 (i.e., 2 + 3 + 5 + 1 = 11).
 *
 * Note:
 *
 * Bonus point if you are able to do this using only O(n) extra space, where n is the total number of rows in the triangle.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
		if triangle.is_empty() {
			return 0;
		}
		let mut cache = vec![0; triangle.len()];
		for (i, row) in triangle.iter().enumerate() {
			let mut prev = 0;
			for (j, e) in row.iter().enumerate() {
				let temp = cache[j];
				cache[j] = match j {
					0 => cache[j],
					j if j < i => std::cmp::min(prev, temp),
					_ => prev,
				} + e;
				prev = temp;
			}
		}
		*cache.iter().min().unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_120() {
		assert_eq!(
			Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
			11
		)
	}
}
