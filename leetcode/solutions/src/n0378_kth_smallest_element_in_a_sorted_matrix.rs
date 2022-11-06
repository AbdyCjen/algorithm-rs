/**
 * [378] Kth Smallest Element in a Sorted Matrix
 *
 * Given an n x n matrix where each of the rows and columns is sorted in ascending order, return the k^th smallest element in the matrix.
 * Note that it is the k^th smallest element in the sorted order, not the k^th distinct element.
 * You must find a solution with a memory complexity better than O(n^2).
 *  
 * <strong class="example">Example 1:
 *
 * Input: matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
 * Output: 13
 * Explanation: The elements in the matrix are [1,5,9,10,11,12,13,<u>13</u>,15], and the 8^th smallest number is 13
 *
 * <strong class="example">Example 2:
 *
 * Input: matrix = [[-5]], k = 1
 * Output: -5
 *
 *  
 * Constraints:
 *
 *     n == matrix.length == matrix[i].length
 *     1 <= n <= 300
 *     -10^9 <= matrix[i][j] <= 10^9
 *     All the rows and columns of matrix are guaranteed to be sorted in non-decreasing order.
 *     1 <= k <= n^2
 *
 *  
 * Follow up:
 *
 *     Could you solve the problem with a constant memory (i.e., O(1) memory complexity)?
 *     Could you solve the problem in O(n) time complexity? The solution may be too advanced for an interview but you may find reading <a href="http://www.cse.yorku.ca/~andy/pubs/X+Y.pdf" target="_blank">this paper</a> fun.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
		let n = mat.len();
		let (mut l, mut r) = (mat[0][0], mat[n - 1][n - 1]);

		while l < r {
			let m = l + (r - l) / 2;
			let mut cnt = 0;
			let mut j = n;
			for row in &mat {
				while j > 0 && row[j - 1] > m {
					j -= 1;
				}
				cnt += j as i32;
			}

			if cnt >= k {
				r = m;
			} else {
				l = m + 1;
			}
		}

		l
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_378() {
		assert_eq!(
			Solution::kth_smallest(matrix![[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8),
			13
		);
		assert_eq!(Solution::kth_smallest(matrix![[-5]], 1), -5);
	}
}
