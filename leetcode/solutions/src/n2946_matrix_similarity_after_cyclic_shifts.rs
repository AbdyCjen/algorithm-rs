/**
 * [2946] Matrix Similarity After Cyclic Shifts
 *
 * You are given a 0-indexed m x n integer matrix mat and an integer k. You have to cyclically right shift odd indexed rows k times and cyclically left shift even indexed rows k times.
 * Return true if the initial and final matrix are exactly the same and false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: mat = [[1,2,1,2],[5,5,5,5],[6,3,6,3]], k = 2
 * Output: true
 * Explanation:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/10/29/similarmatrix.png" style="width: 500px; height: 117px;" />
 * Initially, the matrix looks like the first figure.
 * Second figure represents the state of the matrix after one right and left cyclic shifts to even and odd indexed rows.
 * Third figure is the final state of the matrix after two cyclic shifts which is similar to the initial matrix.
 * Therefore, return true.
 *
 * <strong class="example">Example 2:
 *
 * Input: mat = [[2,2],[2,2]], k = 3
 * Output: true
 * Explanation: As all the values are equal in the matrix, even after performing cyclic shifts the matrix will remain the same. Therefeore, we return true.
 *
 * <strong class="example">Example 3:
 *
 * Input: mat = [[1,2]], k = 1
 * Output: false
 * Explanation: After one cyclic shift, mat = [[2,1]] which is not equal to the initial matrix. Therefore we return false.
 *
 *  
 * Constraints:
 *
 *     1 <= mat.length <= 25
 *     1 <= mat[i].length <= 25
 *     1 <= mat[i][j] <= 25
 *     1 <= k <= 50
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
		let len = mat[0].len() as i32;
		let k = k % len;
		for (i, row) in (0..).zip(mat) {
			let new = if i % 2 == 0 {
				let mut new = row[k as usize..].to_owned();
				new.extend(row[..k as usize].iter().copied());
				new
			} else {
				let k = len - k;
				let mut new = row[k as usize..].to_owned();
				new.extend(row[..k as usize].iter().copied());
				new
			};
			if new != row {
				return false;
			}
		}
		true
	}
}

// submission codes end
