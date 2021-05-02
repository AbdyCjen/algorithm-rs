/**
 * [1062] Partition Array Into Three Parts With Equal Sum
 *
 * Given an array A of integers, return true if and only if we can partition the array into three non-empty parts with equal sums.
 * Formally, we can partition the array if we can find indexes i+1 < j with (A[0] + A[1] + ... + A[i] == A[i+1] + A[i+2] + ... + A[j-1] == A[j] + A[j-1] + ... + A[A.length - 1])
 *  
 * Example 1:
 *
 * Input: A = [0,2,1,-6,6,-7,9,1,2,0,1]
 * Output: true
 * Explanation: 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
 *
 * Example 2:
 *
 * Input: A = [0,2,1,-6,6,7,9,-1,2,0,1]
 * Output: false
 *
 * Example 3:
 *
 * Input: A = [3,3,6,5,-2,2,5,1,-9,4]
 * Output: true
 * Explanation: 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
 *
 *  
 * Constraints:
 *
 *     3 <= A.length <= 50000
 *     -10^4 <= A[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
		let s: i32 = a.iter().sum();
		if s % 3 != 0 || a.is_empty() {
			return false;
		}

		let part_sum = s / 3;
		let mut it = a.iter().copied();
		let acc_it = std::iter::successors(Some(0), move |i| Some(it.next()? + i));

		acc_it.fold(
			part_sum,
			|acc, sum| if acc == sum { acc + part_sum } else { acc },
		) == s + part_sum
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1062() {
		assert_eq!(
			Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
			true
		);
		assert_eq!(
			Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
			false
		);
		assert_eq!(
			Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
			true
		);
		assert_eq!(
			Solution::can_three_parts_equal_sum(vec![3, 1, -1, 6]),
			false
		);
		assert_eq!(
			Solution::can_three_parts_equal_sum(vec![1, -1, 1, -1]),
			false
		);
	}
}
