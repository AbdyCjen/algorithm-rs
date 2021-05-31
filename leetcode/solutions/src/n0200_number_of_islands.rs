/**
 * [200] Number of Islands
 *
 * Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
 *
 * Example 1:
 *
 *
 * Input:
 * 11110
 * 11010
 * 11000
 * 00000
 *
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input:
 * 11000
 * 11000
 * 00100
 * 00011
 *
 * Output: 3
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;
#[allow(dead_code)]
impl Solution {
	fn find(set_slc: &mut [usize], i: usize) -> usize {
		if set_slc[i] != i {
			set_slc[i] = Self::find(set_slc, set_slc[i]);
		}
		set_slc[i]
	}
	fn union(set_slc: &mut [usize], i: usize, j: usize) {
		let (a, b) = (Self::find(set_slc, i), Self::find(set_slc, j));
		if a != b {
			set_slc[a] = b;
		}
	}
	pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
		if grid.is_empty() {
			return 0;
		}
		let (rl, cl) = (grid.len(), grid[0].len());
		let map_idx = |i: usize, j: usize| rl * j + i;
		let by_idx = |i: usize| grid[i % rl][i / rl];
		let mut set_slc = vec![0_usize; grid.len() * grid[0].len()];
		for (i, s) in set_slc.iter_mut().enumerate() {
			*s = i;
		}
		for i in 0..rl {
			for j in 0..cl {
				if '1' == grid[i][j] {
					if i < rl - 1 && grid[i + 1][j] == '1' {
						Self::union(&mut set_slc, map_idx(i, j), map_idx(i + 1, j));
					}
					if j < cl - 1 && grid[i][j + 1] == '1' {
						Self::union(&mut set_slc, map_idx(i, j), map_idx(i, j + 1));
					}
				}
			}
		}
		let mut sset = HashSet::<usize>::new();
		for i in 0..set_slc.len() {
			if by_idx(i) == '1' {
				sset.insert(Self::find(&mut set_slc, i));
			}
		}
		sset.len() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_200() {
		assert_eq!(
			Solution::num_islands(vec![
				vec!['1', '1', '1', '1', '0'],
				vec!['1', '1', '0', '1', '0'],
				vec!['1', '1', '0', '0', '0'],
				vec!['0', '0', '0', '0', '0']
			]),
			1
		);
		assert_eq!(
			Solution::num_islands(vec![
				vec!['1', '1', '0', '0', '0'],
				vec!['1', '1', '0', '0', '0'],
				vec!['0', '0', '1', '0', '0'],
				vec!['0', '0', '0', '1', '1']
			]),
			3
		);
		assert_eq!(Solution::num_islands(vec![]), 0);
	}
}
