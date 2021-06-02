/**
 * [1860] Find Kth Largest XOR Coordinate Value
 *
 * You are given a 2D matrix of size m x n, consisting of non-negative integers. You are also given an integer k.
 * The value of coordinate (a, b) of the matrix is the XOR of all matrix[i][j] where 0 <= i <= a < m and 0 <= j <= b < n (0-indexed).
 * Find the k^th largest value (1-indexed) of all the coordinates of matrix.
 *  
 * Example 1:
 *
 * Input: matrix = [[5,2],[1,6]], k = 1
 * Output: 7
 * Explanation: The value of coordinate (0,1) is 5 XOR 2 = 7, which is the largest value.
 * Example 2:
 *
 * Input: matrix = [[5,2],[1,6]], k = 2
 * Output: 5
 * Explanation: The value of coordinate (0,0) is 5 = 5, which is the 2nd largest value.
 * Example 3:
 *
 * Input: matrix = [[5,2],[1,6]], k = 3
 * Output: 4
 * Explanation: The value of coordinate (1,0) is 5 XOR 1 = 4, which is the 3rd largest value.
 * Example 4:
 *
 * Input: matrix = [[5,2],[1,6]], k = 4
 * Output: 0
 * Explanation: The value of coordinate (1,1) is 5 XOR 2 XOR 1 XOR 6 = 0, which is the 4th largest value.
 *  
 * Constraints:
 *
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m, n <= 1000
 *     0 <= matrix[i][j] <= 10^6
 *     1 <= k <= m * n
 *
 */
pub struct Solution {}

// submission codes start here

extern crate rand;
use rand::Rng;
#[allow(dead_code)]
impl Solution {
	pub fn kth_largest_value(mat: Vec<Vec<i32>>, k: i32) -> i32 {
		fn select(nums: &mut [i32], r: usize) -> i32 {
			let (mut i, mut j, mut k) = (0, 0, nums.len() - 1);
			let ax = nums[rand::thread_rng().gen_range(0..nums.len())];
			while j <= k {
				use std::cmp::Ordering;
				match nums[j].cmp(&ax) {
					Ordering::Equal => j += 1,
					Ordering::Less => {
						nums.swap(i, j);
						i += 1;
						j += 1;
					}
					Ordering::Greater => {
						nums.swap(j, k);
						k -= 1;
					}
				}
			}
			match r {
				r if r < i => select(&mut nums[..i], r),
				r if r > k => select(&mut nums[k + 1..], r - k - 1),
				r => nums[r],
			}
		}

		let m = mat.len();
		let n = mat[0].len();
		let mut arr = vec![0; m * n];
		let idx = |i, j| -> usize { i * n + j };
		arr[0] = mat[0][0];
		for i in 1..m {
			arr[idx(i, 0)] = mat[i][0] ^ arr[idx(i - 1, 0)];
		}
		for j in 1..n {
			arr[idx(0, j)] = mat[0][j] ^ arr[idx(0, j - 1)];
		}
		for (i, row) in (0..m).zip(mat).skip(1) {
			for (j, blk) in (0..n).zip(row).skip(1) {
				arr[idx(i, j)] =
					blk ^ arr[idx(i - 1, j)] ^ arr[idx(i, j - 1)] ^ arr[idx(i - 1, j - 1)];
			}
		}
		select(&mut arr, m * n - k as usize)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1860() {
		assert_eq!(
			Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1),
			7
		);
		assert_eq!(
			Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2),
			5
		);
		assert_eq!(
			Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3),
			4
		);
		assert_eq!(
			Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 4),
			0
		);

		assert_eq!(
			Solution::kth_largest_value(
				vec![vec![10, 9, 5], vec![2, 0, 4], vec![1, 0, 9], vec![3, 4, 8]],
				10
			),
			1
		);
	}
}
