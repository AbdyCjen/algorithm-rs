/**
 * [1444] Number of Ways of Cutting a Pizza
 *
 * Given a rectangular pizza represented as a rows x cols matrix containing the following characters: 'A' (an apple) and '.' (empty cell) and given the integer k. You have to cut the pizza into k pieces using k-1 cuts.
 *
 * For each cut you choose the direction: vertical or horizontal, then you choose a cut position at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part of the pizza to a person. Give the last piece of pizza to the last person.
 *
 * Return the number of ways of cutting the pizza such that each piece contains at least one apple. Since the answer can be a huge number, return this modulo 10^9 + 7.
 *
 *  
 * <strong class="example">Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/ways_to_cut_apple_1.png" style="width: 500px; height: 378px;" />
 *
 *
 * Input: pizza = ["A..","AAA","..."], k = 3
 * Output: 3
 * Explanation: The figure above shows the three ways to cut the pizza. Note that pieces must contain at least one apple.
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: pizza = ["A..","AA.","..."], k = 3
 * Output: 1
 *
 *
 * <strong class="example">Example 3:
 *
 *
 * Input: pizza = ["A..","A..","..."], k = 1
 * Output: 1
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= rows, cols <= 50
 *     rows == pizza.length
 *     cols == pizza[i].length
 *     1 <= k <= 10
 *     pizza consists of characters 'A' and '.' only.
 *
 */
pub struct Solution {}

impl Solution {
	pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
		const MO: i32 = 1e9 as i32 + 7;
		let (m, n) = (pizza.len(), pizza[0].len());
		let mut sum = vec![vec![0; n]; m];
		let mut mat = vec![vec![vec![-1; k as usize]; n]; m];

		for (r, pr) in sum.iter_mut().zip(pizza) {
			let mut prv = 0;
			for (b, pb) in r.iter_mut().zip(pr.bytes()).rev() {
				prv += (pb == b'A') as i32;
				*b = prv;
			}
		}
		for j in 0..n {
			for i in (0..m - 1).rev() {
				sum[i][j] += sum[i + 1][j];
			}
		}

		for (r, pr) in mat.iter_mut().zip(&sum) {
			for (b, &pb) in r.iter_mut().zip(pr) {
				b[0] = (pb > 0) as i32;
			}
		}

		for k in 1..k as usize {
			for i in 0..m {
				for j in 0..n {
					let mut ans = 0;
					for ii in i + 1..m {
						match sum[ii][j] {
							0 => break,
							ss if ss < sum[i][j] => ans = (ans + mat[ii][j][k - 1]) % MO,
							_ => {}
						}
					}
					for jj in j + 1..n {
						match sum[i][jj] {
							0 => break,
							ss if ss < sum[i][j] => ans = (ans + mat[i][jj][k - 1]) % MO,
							_ => {}
						}
					}
					mat[i][j][k] = ans;
				}
			}
		}
		mat[0][0][k as usize - 1]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1444() {
		assert_eq!(Solution::ways(vec_string!["A..", "AAA", "..."], 3), 3);
	}
}
