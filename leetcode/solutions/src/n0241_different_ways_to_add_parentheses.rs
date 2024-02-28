/**
 * [241] Different Ways to Add Parentheses
 *
 * Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.
 * The test cases are generated such that the output values fit in a 32-bit integer and the number of different results does not exceed 10^4.
 *  
 * <strong class="example">Example 1:
 *
 * Input: expression = "2-1-1"
 * Output: [0,2]
 * Explanation:
 * ((2-1)-1) = 0
 * (2-(1-1)) = 2
 *
 * <strong class="example">Example 2:
 *
 * Input: expression = "2*3-4*5"
 * Output: [-34,-14,-10,-10,10]
 * Explanation:
 * (2*(3-(4*5))) = -34
 * ((2*3)-(4*5)) = -14
 * ((2*(3-4))*5) = -10
 * (2*((3-4)*5)) = -10
 * (((2*3)-4)*5) = 10
 *
 *  
 * Constraints:
 *
 *     1 <= expression.length <= 20
 *     expression consists of digits and the operator '+', '-', and '*'.
 *     All the integer values in the input expression are in the range [0, 99].
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn diff_ways_to_compute(expr: String) -> Vec<i32> {
		let mut cur = 0;
		let mut op = vec![];
		let mut nums = vec![];
		for c in expr.bytes() {
			match c {
				b'-' | b'+' | b'*' => {
					nums.push(cur);
					cur = 0;
					op.push(c);
				}
				_ => cur = cur * 10 + (c - b'0') as i32,
			};
		}
		nums.push(cur);
		Self::solve(&op, &nums).into_iter().collect()
	}

	fn solve(ops: &[u8], nums: &[i32]) -> Vec<i32> {
		if ops.is_empty() {
			return nums.to_owned();
		}
		let mut ans = vec![];
		for (i, &op) in (0..).zip(ops) {
			let l = Self::solve(&ops[..i], &nums[..=i]);
			let r = Self::solve(&ops[i + 1..], &nums[i + 1..]);
			for l in l {
				for &r in &r {
					let a = match op {
						b'+' => l + r,
						b'-' => l - r,
						_ => l * r,
					};
					ans.push(a);
				}
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_241() {
		assert_eq!(Solution::diff_ways_to_compute("2-1-1".to_owned()), [2, 0]);
		assert_eq!(
			Solution::diff_ways_to_compute("2*3-4*5".to_owned()),
			[-34, -10, -14, -10, 10]
		);
	}
}
