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
/*
  把每行扫成一个柱状图, 然后用84里的算法求解就好;
*/

impl Solution {
	pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
		if matrix.len() == 0 {
			return 0;
		}
		let mut max_area = 0;
		let mut row_buf = vec![0_i32; matrix[0].len()];
		for row in matrix.into_iter() {
			for (&c, col) in row.iter().zip(row_buf.iter_mut()) {
				*col = if c == '0' { 0 } else { *col + 1 };
			}
			max_area = std::cmp::max(max_area, Solution::largest_rectangle_area(&mut row_buf));
		}
		max_area
	}

	fn largest_rectangle_area(heights: &mut Vec<i32>) -> i32 {
		if heights.len() == 0 {
			return 0;
		}
		let mut st = vec![(0, heights[0])];
		let mut max_area: i32 = heights[0];
		heights.push(0);
		for (i, &h) in heights.iter().enumerate().skip(1) {
			while let Some(&(_, th)) = st.iter().rev().next() {
				if !(th >= h) {
					break;
				}
				st.pop();
				max_area = match st.iter().rev().next() {
					None => std::cmp::max(i as i32 * th, max_area),
					Some(&(ni, _)) => std::cmp::max((i - ni - 1) as i32 * th, max_area),
				}
			}
			st.push((i, h));
		}
		max_area
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
