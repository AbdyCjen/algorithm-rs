/**
 * [85] Maximal Rectangle
 *
 * Given a 2D binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   ['1','0','1','0','0'],
 *   ['1','0','1','1','1'],
 *   ['1','1','1','1','1'],
 *   ['1','0','0','1','0']
 * ]
 * Output: 6
 *
 *
 */
pub struct Solution {}

// submission codes start here
impl Solution {
	// FIXME: O(n) solution
	pub fn maximal_rectangle(mat: Vec<Vec<char>>) -> i32 {
		let mut cnts = vec![0; mat[0].len()];
		let mut ans = 0;
		for row in mat {
			for (i, &c) in (0..).zip(&row) {
				match c {
					'0' => cnts[i] = 0,
					_ => cnts[i] += 1,
				}
				let mut h = cnts[i];
				ans = ans.max(h);
				for (w, &c) in (0..i as i32).zip(&cnts[..i]).rev() {
					h = h.min(c);
					ans = ans.max(h * w);
				}
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_85() {
		assert_eq!(
			Solution::maximal_rectangle(vec![
				vec!['1', '0', '1', '0', '0'],
				vec!['1', '0', '1', '1', '1'],
				vec!['1', '1', '1', '1', '1'],
				vec!['1', '0', '0', '1', '0']
			]),
			6
		);
	}
}
