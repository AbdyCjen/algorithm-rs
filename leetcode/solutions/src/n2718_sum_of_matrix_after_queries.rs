/**
 * [2718] Sum of Matrix After Queries
 *
 * You are given an integer n and a 0-indexed 2D array queries where queries[i] = [typei, indexi, vali].
 * Initially, there is a 0-indexed n x n matrix filled with 0's. For each query, you must apply one of the following changes:
 *
 *     if typei == 0, set the values in the row with indexi to vali, overwriting any previous values.
 *     if typei == 1, set the values in the column with indexi to vali, overwriting any previous values.
 *
 * Return the sum of integers in the matrix after all queries are applied.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/05/11/exm1.png" style="width: 681px; height: 161px;" />
 * Input: n = 3, queries = [[0,0,1],[1,2,2],[0,2,3],[1,0,4]]
 * Output: 23
 * Explanation: The image above describes the matrix after each query. The sum of the matrix after all queries are applied is 23.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/05/11/exm2.png" style="width: 681px; height: 331px;" />
 * Input: n = 3, queries = [[0,0,4],[0,1,2],[1,0,1],[0,2,3],[1,2,1]]
 * Output: 17
 * Explanation: The image above describes the matrix after each query. The sum of the matrix after all queries are applied is 17.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^4
 *     1 <= queries.length <= 5 * 10^4
 *     queries[i].length == 3
 *     0 <= typei <= 1
 *     0 <= indexi < n
 *     0 <= vali <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
		let (mut row, mut col) = (vec![(0, 0); n as usize], vec![(0, 0); n as usize]);
		for (q, i) in queries.into_iter().zip((2..).step_by(2)) {
			if q[0] == 0 {
				row[q[1] as usize] = (q[2], i);
			} else {
				col[q[1] as usize] = (q[2] as i64, i);
			}
		}
		col.sort_unstable_by_key(|v| v.1);
		for i in 0..n as usize - 1 {
			col[i + 1].0 += col[i].0;
		}
		let tot = col[n as usize - 1].0;
		row.into_iter()
			.map(|(rv, ridx)| {
				let cidx = col.binary_search_by_key(&(ridx + 1), |v| v.1).unwrap_err();
				if cidx == 0 {
					tot
				} else {
					rv as i64 * cidx as i64 + tot - col[cidx - 1].0
				}
			})
			.sum()
	}
}

// submission codes end
