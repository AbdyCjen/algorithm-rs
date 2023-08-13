/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
 *
 *     answer[i] % answer[j] == 0, or
 *     answer[j] % answer[i] == 0
 *
 * If there are multiple solutions, return any of them.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: [1,2]
 * Explanation: [1,3] is also accepted.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,4,8]
 * Output: [1,2,4,8]
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 1000
 *     1 <= nums[i] <= 2 * 10^9
 *     All the integers in nums are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
		nums.sort_unstable();
		let mut cache = vec![(0, -1); nums.len()];
		let mut next = Self::solve(&nums, 1, &mut cache);
		let mut ans = vec![next.0];
		while next.1 > 0 {
			let idx = nums.binary_search(&next.0).unwrap();
			next = cache[idx];
			ans.push(next.0);
		}
		ans
	}
	fn solve(nums: &[i32], n: i32, cache: &mut [(i32, i32)]) -> (i32, i32) {
		match nums {
			[] => (n, 0),
			[i, rest @ ..] => {
				let mut ans = Self::solve(rest, n, cache);
				if *i % n == 0 {
					if cache[0].1 < 0 {
						cache[0] = Self::solve(rest, *i, &mut cache[1..]);
					}
					if cache[0].1 + 1 > ans.1 {
						ans = (*i, cache[0].1 + 1);
					}
				}
				ans
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_368() {
		assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 3]), [1, 2]);
	}
}
