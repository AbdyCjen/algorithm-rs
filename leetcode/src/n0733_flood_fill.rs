/**
 * [733] Flood Fill
 *
 *
 * An image is represented by a 2-D array of integers, each integer representing the pixel value of the image (from 0 to 65535).
 *
 * Given a coordinate (sr, sc) representing the starting pixel (row and column) of the flood fill, and a pixel value newColor, "flood fill" the image.
 *
 * To perform a "flood fill", consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color as the starting pixel), and so on.  Replace the color of all of the aforementioned pixels with the newColor.
 *
 * At the end, return the modified image.
 *
 * Example 1:<br />
 *
 * Input:
 * image = [[1,1,1],[1,1,0],[1,0,1]]
 * sr = 1, sc = 1, newColor = 2
 * Output: [[2,2,2],[2,2,0],[2,0,1]]
 * Explanation:
 * From the center of the image (with position (sr, sc) = (1, 1)), all pixels connected
 * by a path of the same color as the starting pixel are colored with the new color.
 * Note the bottom corner is not colored 2, because it is not 4-directionally connected
 * to the starting pixel.
 *
 *
 *
 * Note:
 * The length of image and image[0] will be in the range [1, 50].
 * The given starting pixel will satisfy 0 <= sr < image.length and 0 <= sc < image[0].length.
 * The value of each color in image[i][j] and newColor will be an integer in [0, 65535].
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::VecDeque;
#[allow(dead_code)]
impl Solution {
	pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
		if image.is_empty() {
			return image;
		}
		let (sr, sc) = (sr as usize, sc as usize);
		let old_color = image[sr][sc];
		image[sr][sc] = new_color;
		if old_color == new_color {
			return image;
		}
		let (rl, cl) = (image.len() as isize, image[0].len() as isize);

		let mut st = VecDeque::new();
		st.push_back((sr as isize, sc as isize));
		while let Some((i, j)) = st.pop_front() {
			for &(i, j) in &[(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
				if i >= 0 && i < rl && j >= 0 && j < cl {
					let blc = &mut image[i as usize][j as usize];
					if *blc == old_color {
						*blc = new_color;
						st.push_back((i, j))
					}
				}
			}
		}
		image
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_733() {
		assert_eq!(
			Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
			vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
		);
	}
}
