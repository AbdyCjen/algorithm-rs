/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is greater than its neighbors.
 *
 * Given an input array nums, where nums[i] &ne; nums[i+1], find a peak element and return its index.
 *
 * The array may contain multiple peaks, in that case return the index to any one of the peaks is fine.
 *
 * You may imagine that nums[-1] = nums[n] = -&infin;.
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 1 or 5
 * Explanation: Your function can return either index number 1 where the peak element is 2,
 *              or index number 5 where the peak element is 6.
 *
 *
 * Note:
 *
 * Your solution should be in logarithmic complexity.
 *
 */
pub struct Solution {}

// submission codes start here

// 只要有递增区间就一定有peak
#[allow(dead_code)]
impl Solution {
	pub fn find_peak_element(nums: Vec<i32>) -> i32 {
		let (mut l, mut r) = (0, nums.len() - 1);
		while l < r {
			let mid = (l + r) / 2;
			if nums[mid] < nums[r] || nums[mid] < *nums.get(mid + 1).unwrap_or(&i32::MAX) {
				l = mid + 1;
			} else {
				r = mid;
			}
		}
		l as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_162() {
		fn test_peak(arr: Vec<i32>) {
			let a = Solution::find_peak_element(arr.clone()) as usize;
			assert!(
				arr[a]
					>= arr[..a]
						.last()
						.max(arr[a + 1..].first())
						.copied()
						.unwrap_or(i32::MIN)
			);
		}
		test_peak(vec![1, 2, 3, 1]);
		test_peak(vec![1, 2, 1, 3, 5, 6]);
	}
}
