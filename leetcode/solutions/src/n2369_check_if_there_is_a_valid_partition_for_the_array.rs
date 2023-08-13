/**
 * [2369] Check if There is a Valid Partition For The Array
 *
 * You are given a 0-indexed integer array nums. You have to partition the array into one or more contiguous subarrays.
 * We call a partition of the array valid if each of the obtained subarrays satisfies one of the following conditions:
 * <ol>
 *     The subarray consists of exactly 2 equal elements. For example, the subarray [2,2] is good.
 *     The subarray consists of exactly 3 equal elements. For example, the subarray [4,4,4] is good.
 *     The subarray consists of exactly 3 consecutive increasing elements, that is, the difference between adjacent elements is 1. For example, the subarray [3,4,5] is good, but the subarray [1,3,5] is not.
 * </ol>
 * Return true if the array has at least one valid partition. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [4,4,4,5,6]
 * Output: true
 * Explanation: The array can be partitioned into the subarrays [4,4] and [4,5,6].
 * This partition is valid, so we return true.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,1,1,2]
 * Output: false
 * Explanation: There is no valid partition for this array.
 *
 *  
 * Constraints:
 *
 *     2 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn valid_partition(nums: Vec<i32>) -> bool {
		Self::solve(&nums, &mut vec![None; nums.len() + 1])
	}

	fn solve(nums: &[i32], cache: &mut [Option<bool>]) -> bool {
		if let Some(ans) = cache[nums.len()] {
			return ans;
		}
		let cond3 = |a, b, c| (a == b && b == c) || (a + 1 == b && b + 1 == c);
		let ans = match nums {
			[] => true,
			[a, b, rest @ ..] if a == b && Self::solve(rest, cache) => true,
			[a, b, c, rest @ ..] if cond3(*a, *b, *c) => Self::solve(rest, cache),
			_ => false,
		};
		cache[nums.len()] = Some(ans);
		ans
	}
}

// submission codes end
