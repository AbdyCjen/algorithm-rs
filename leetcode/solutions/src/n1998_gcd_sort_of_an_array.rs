/**
 * [1998] GCD Sort of an Array
 *
 * You are given an integer array nums, and you can perform the following operation any number of times on nums:
 *
 *     Swap the positions of two elements nums[i] and nums[j] if gcd(nums[i], nums[j]) > 1 where gcd(nums[i], nums[j]) is the greatest common divisor of nums[i] and nums[j].
 *
 * Return true if it is possible to sort nums in non-decreasing order using the above swap method, or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [7,21,3]
 * Output: true
 * Explanation: We can sort [7,21,3] by performing the following operations:
 * - Swap 7 and 21 because gcd(7,21) = 7. nums = [<u>21</u>,<u>7</u>,3]
 * - Swap 21 and 3 because gcd(21,3) = 3. nums = [<u>3</u>,7,<u>21</u>]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [5,2,6,2]
 * Output: false
 * Explanation: It is impossible to sort the array because 5 cannot be swapped with any other element.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [10,5,9,3,15]
 * Output: true
 * We can sort [10,5,9,3,15] by performing the following operations:
 * - Swap 10 and 15 because gcd(10,15) = 5. nums = [<u>15</u>,5,9,3,<u>10</u>]
 * - Swap 15 and 3 because gcd(15,3) = 3. nums = [<u>3</u>,5,9,<u>15</u>,10]
 * - Swap 10 and 15 because gcd(10,15) = 5. nums = [3,5,9,<u>10</u>,<u>15</u>]
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 3 * 10^4
 *     2 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn gcd_sort(nums: Vec<i32>) -> bool {
		fn find(set: &mut [i32], x: i32) -> i32 {
			if set[x as usize] != x {
				set[x as usize] = find(set, set[x as usize]);
			}
			set[x as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}
		// primes in 0..1e5.sqrt()
		const PRIMES: &[i32] = &[
			2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
			89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
			181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
			277, 281, 283, 293, 307, 311, 313, 317,
		];
		let max = nums.iter().copied().max().unwrap();
		let mut set: Vec<i32> = (0..=max).collect();
		for &n in &nums {
			let mut nn = n;
			for &pr in PRIMES.iter().take_while(move |&pr| pr * pr <= n) {
				if nn % pr == 0 {
					union(&mut set, nn, pr);
					while nn % pr == 0 {
						nn /= pr;
					}
				}
				if nn > 1 {
					union(&mut set, nn, n);
				}
			}
		}
		let mut sorted = nums.clone();
		sorted.sort_unstable();
		sorted
			.into_iter()
			.zip(nums)
			.all(|(s, n)| find(&mut set, s) == find(&mut set, n))
	}
}

// submission codes end
