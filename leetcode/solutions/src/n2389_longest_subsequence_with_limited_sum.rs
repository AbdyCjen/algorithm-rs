/**
 * [2389] Longest Subsequence With Limited Sum
 *
 * You are given an integer array nums of length n, and an integer array queries of length m.
 * Return an array answer of length m where answer[i] is the maximum size of a subsequence that you can take from nums such that the sum of its elements is less than or equal to queries[i].
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [4,5,2,1], queries = [3,10,21]
 * Output: [2,3,4]
 * Explanation: We answer the queries as follows:
 * - The subsequence [2,1] has a sum less than or equal to 3. It can be proven that 2 is the maximum size of such a subsequence, so answer[0] = 2.
 * - The subsequence [4,5,1] has a sum less than or equal to 10. It can be proven that 3 is the maximum size of such a subsequence, so answer[1] = 3.
 * - The subsequence [4,5,2,1] has a sum less than or equal to 21. It can be proven that 4 is the maximum size of such a subsequence, so answer[2] = 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,3,4,5], queries = [1]
 * Output: [0]
 * Explanation: The empty subsequence is the only subsequence that has a sum less than or equal to 1, so answer[0] = 0.
 *  
 * Constraints:
 *
 *     n == nums.length
 *     m == queries.length
 *     1 <= n, m <= 1000
 *     1 <= nums[i], queries[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
		nums.sort_unstable();
		let mut prv = nums[0];
		for n in nums.iter_mut().skip(1) {
			*n += prv;
			prv = *n;
		}

		queries
			.into_iter()
			.map(|q| match nums.binary_search(&q) {
				Err(i) => i as i32,
				Ok(i) => 1 + i as i32,
			})
			.collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2389() {
		assert_eq!(
			Solution::answer_queries(
				vec![736411, 184882, 914641, 37925, 214915],
				vec![718089, 665450]
			),
			vec![3, 3]
		);
	}
}
