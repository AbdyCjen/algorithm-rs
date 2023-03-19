/**
 * [313] Super Ugly Number
 *
 * A super ugly number is a positive integer whose prime factors are in the array primes.
 * Given an integer n and an array of integers primes, return the n^th super ugly number.
 * The n^th super ugly number is guaranteed to fit in a 32-bit signed integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 12, primes = [2,7,13,19]
 * Output: 32
 * Explanation: [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12 super ugly numbers given primes = [2,7,13,19].
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1, primes = [2,3,5]
 * Output: 1
 * Explanation: 1 has no prime factors, therefore all of its prime factors are in the array primes = [2,3,5].
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *     1 <= primes.length <= 100
 *     2 <= primes[i] <= 1000
 *     primes[i] is guaranteed to be a prime number.
 *     All the values of primes are unique and sorted in ascending order.
 *
 */
pub struct Solution {}

impl Solution {
	pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
		let mut nums = vec![1_i32];
		let mut idx = vec![0; primes.len()];
		let mut next = std::collections::BTreeMap::new();
		next.insert(1, (0..primes.len()).collect::<Vec<_>>());
		for _ in 0..n {
			let ne = *next.keys().next().unwrap();
			nums.push(ne);
			for i in next.remove(&ne).into_iter().flatten() {
				idx[i] += 1;
				next.entry(nums[idx[i]].saturating_mul(primes[i]))
					.or_default()
					.push(i);
			}
		}
		nums.pop().unwrap()
	}
}
