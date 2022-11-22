/**
 * [279] Perfect Squares
 *
 * Given an integer n, return the least number of perfect square numbers that sum to n.
 * A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 12
 * Output: 3
 * Explanation: 12 = 4 + 4 + 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 13
 * Output: 2
 * Explanation: 13 = 4 + 9.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	//bfs
	pub fn num_squares(n: i32) -> i32 {
		let sqs = (1..)
			.map(|i| i * i)
			.take_while(|&sq| sq <= n)
			.collect::<Vec<_>>();

		let mut dq = vec![n];
		let mut visited = vec![false; n as usize + 1];
		let mut i = 0;
		while !dq.is_empty() {
			for o in std::mem::take(&mut dq) {
				match sqs.binary_search(&o) {
					Ok(_) => return i + 1,
					Err(j) => {
						for sq in &sqs[..j] {
							let next = o - sq;
							if !visited[next as usize] {
								visited[next as usize] = true;
								dq.push(next);
							}
						}
					}
				}
			}
			i += 1;
		}
		unreachable!()
	}
	pub fn num_squares_01(n: i32) -> i32 {
		Self::solve(
			n,
			&(1..)
				.map(|i| i * i)
				.take_while(|&sq| sq <= n)
				.collect::<Vec<_>>(),
			&mut HashMap::new(),
		)
	}

	fn solve(n: i32, sqs: &[i32], cache: &mut HashMap<i32, i32>) -> i32 {
		if let Some(ans) = cache.get(&n) {
			return *ans;
		} else if n < 4 {
			return n;
		}

		let mut ans = i32::MAX;
		match sqs.binary_search(&n) {
			Ok(_) => return 1,
			Err(i) => {
				let sqs = &sqs[..i];
				for sq in sqs {
					ans = ans.min(n / sq + Self::solve(n % sq, sqs, cache));
				}
			}
		}
		cache.insert(n, ans);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_279() {
		assert_eq!(Solution::num_squares(13), 2);
		assert_eq!(Solution::num_squares(12), 3);
	}
}
