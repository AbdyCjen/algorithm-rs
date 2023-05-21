/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate an n x n matrix filled with elements from 1 to n^2 in spiral order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg" style="width: 242px; height: 242px;" />
 * Input: n = 3
 * Output: [[1,2,3],[8,9,4],[7,6,5]]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 20
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
		let mut cur = 1;
		let mut ans = vec![vec![0; n as usize]; n as usize];
		for i in 0..(n + 1) / 2 {
			let (mut pos, len) = ((i, i), n - i * 2 - 1);
			ans[pos.0 as usize][pos.1 as usize] = cur;
			for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
				for _ in 0..len {
					ans[pos.0 as usize][pos.1 as usize] = cur;
					cur += 1;
					pos = (pos.0 + dir.0, pos.1 + dir.1);
				}
			}
		}
		ans
	}

	pub fn generate_matrix1(n: i32) -> Vec<Vec<i32>> {
		let mut cur = 1..;
		let n = n as usize;
		let mut ans = vec![vec![0; n]; n];
		let mut it = ans.iter_mut();
		let mut start = 0;
		let set = |(i, n): (i32, &mut i32)| *n = i;
		while let (Some(head), tail) = (it.next(), it.next_back()) {
			let (end, len, cur) = (n - start, n - start * 2, cur.by_ref());
			cur.take(len).zip(&mut head[start..end]).for_each(set);
			let mat: &mut [_] = it.into_slice();
			cur.take(mat.len())
				.zip(mat.iter_mut().map(|v| &mut v[end - 1]))
				.for_each(set);

			if let Some(tail) = tail {
				cur.take(len)
					.zip(tail[start..end].iter_mut().rev())
					.for_each(set);
			}
			cur.take(mat.len())
				.zip(mat.iter_mut().rev().map(|v| &mut v[start]))
				.for_each(set);
			it = mat.iter_mut();
			start += 1;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_59() {
		assert_eq!(Solution::generate_matrix(1), matrix!([1]));
		assert_eq!(
			Solution::generate_matrix(3),
			matrix![[1, 2, 3], [8, 9, 4], [7, 6, 5]]
		);
		assert_eq!(
			Solution::generate_matrix(4),
			matrix![
				[1, 2, 3, 4],
				[12, 13, 14, 5],
				[11, 16, 15, 6],
				[10, 9, 8, 7]
			]
		);
	}
}
