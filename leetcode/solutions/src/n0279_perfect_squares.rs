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

impl Solution {
	//bfs
	pub fn num_squares(n: i32) -> i32 {
		let sqs = (1..)
			.map(|i| i * i)
			.take_while(|&sq| sq <= n)
			.collect::<Vec<_>>();
		let mut dq = vec![n];
		let mut visited = vec![false; n as usize + 1];
		let mut i = 1;
		'outer: loop {
			for no in std::mem::take(&mut dq) {
				for &sq in &sqs {
					match no - sq {
						0 => break 'outer i,
						i32::MIN..=0 => break,
						no @ 1.. if !visited[no as usize] => {
							visited[no as usize] = true;
							dq.push(no);
						}
						_ => {}
					}
				}
			}
			i += 1;
		}
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
