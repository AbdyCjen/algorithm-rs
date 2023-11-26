/**
 * [1685] Sum of Absolute Differences in a Sorted Array
 *
 * You are given an integer array nums sorted in non-decreasing order.
 * Build and return an integer array result with the same length as nums such that result[i] is equal to the summation of absolute differences between nums[i] and all the other elements in the array.
 * In other words, result[i] is equal to sum(|nums[i]-nums[j]|) where 0 <= j < nums.length and j != i (0-indexed).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [2,3,5]
 * Output: [4,3,5]
 * Explanation: Assuming the arrays are 0-indexed, then
 * result[0] = |2-2| + |2-3| + |2-5| = 0 + 1 + 3 = 4,
 * result[1] = |3-2| + |3-3| + |3-5| = 1 + 0 + 2 = 3,
 * result[2] = |5-2| + |5-3| + |5-5| = 3 + 2 + 0 = 5.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,4,6,8,10]
 * Output: [24,15,13,15,21]
 * 
 *  
 * Constraints:
 * 
 *     2 <= nums.length <= 10^5
 *     1 <= nums[i] <= nums[i + 1] <= 10^4
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = nums.clone();
        for i in 1..sum.len() {
            sum[i] += sum[i - 1];
        }
        let tot = sum[nums.len() - 1];
        let len = sum.len() as i32;
        for ((i, s), n) in (1..).zip(&mut sum).zip(nums) {
            let ans = (tot - *s) - n * (len - i) + n * i - *s;
            *s = ans;
        }
        sum
    }
}

// submission codes end
