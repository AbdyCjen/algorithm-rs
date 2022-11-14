/**
 * [26] Remove Duplicates from Sorted Array
 *
 * Given an integer array nums sorted in non-decreasing order, remove the duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that each unique element appears only once. The relative order of the elements should be kept the same.
 * Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
 * Return k after placing the final result in the first k slots of nums.
 * Do not allocate extra space for another array. You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * Custom Judge:
 * The judge will test your solution with the following code:
 *
 * int[] nums = [...]; // Input array
 * int[] expectedNums = [...]; // The expected answer with correct length
 * int k = removeDuplicates(nums); // Calls your implementation
 * assert k == expectedNums.length;
 * for (int i = 0; i < k; i++) {
 *     assert nums[i] == expectedNums[i];
 * }
 *
 * If all assertions pass, then your solution will be accepted.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,1,2]
 * Output: 2, nums = [1,2,_]
 * Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
 * It does not matter what you leave beyond the returned k (hence they are underscores).
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,0,1,1,1,2,2,3,3,4]
 * Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
 * Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
 * It does not matter what you leave beyond the returned k (hence they are underscores).
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 3 * 10^4
 *     -100 <= nums[i] <= 100
 *     nums is sorted in non-decreasing order.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
		let mut i = 0;
		for j in 1..nums.len() {
			if nums[j] != nums[i] {
				i += 1;
				nums[i] = nums[j];
			}
		}
		nums.truncate(i + 1);
		nums.len() as _
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_26() {
		let mut case = vec![1, 1, 2];
		let exp = [1, 2];
		assert_eq!(Solution::remove_duplicates(&mut case), exp.len() as i32);
		assert_eq!(case, exp);

		let mut case = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
		let exp = [0, 1, 2, 3, 4];
		assert_eq!(Solution::remove_duplicates(&mut case), exp.len() as i32);
		assert_eq!(case, exp);
	}
}
