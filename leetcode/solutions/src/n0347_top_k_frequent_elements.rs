/**
 * [347] Top K Frequent Elements
 *
 * Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,1,1,2,2,3], k = 2
 * Output: [1,2]
 * <strong class="example">Example 2:
 * Input: nums = [1], k = 1
 * Output: [1]
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     -10^4 <= nums[i] <= 10^4
 *     k is in the range [1, the number of unique elements in the array].
 *     It is guaranteed that the answer is unique.
 *
 *  
 * Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
		let mut cnt = std::collections::HashMap::new();
		for n in nums {
			*cnt.entry(n).or_insert(0) += 1;
		}

		let mut cnt = cnt.into_iter().collect::<Vec<_>>();
		cnt.sort_unstable_by(|a, b| b.1.cmp(&a.1));
		cnt.into_iter().map(|i| i.0).take(k as usize).collect()
	}
}

// submission codes end
