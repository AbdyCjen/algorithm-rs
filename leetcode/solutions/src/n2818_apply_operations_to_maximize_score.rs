/**
 * [2818] Apply Operations to Maximize Score
 *
 * You are given an array nums of n positive integers and an integer k.
 * Initially, you start with a score of 1. You have to maximize your score by applying the following operation at most k times:
 *
 *     Choose any non-empty subarray nums[l, ..., r] that you haven't chosen previously.
 *     Choose an element x of nums[l, ..., r] with the highest prime score. If multiple such elements exist, choose the one with the smallest index.
 *     Multiply your score by x.
 *
 * Here, nums[l, ..., r] denotes the subarray of nums starting at index l and ending at the index r, both ends being inclusive.
 * The prime score of an integer x is equal to the number of distinct prime factors of x. For example, the prime score of 300 is 3 since 300 = 2 * 2 * 3 * 5 * 5.
 * Return the maximum possible score after applying at most k operations.
 * Since the answer may be large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [8,3,9,3,8], k = 2
 * Output: 81
 * Explanation: To get a score of 81, we can apply the following operations:
 * - Choose subarray nums[2, ..., 2]. nums[2] is the only element in this subarray. Hence, we multiply the score by nums[2]. The score becomes 1 * 9 = 9.
 * - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 1, but nums[2] has the smaller index. Hence, we multiply the score by nums[2]. The score becomes 9 * 9 = 81.
 * It can be proven that 81 is the highest score one can obtain.
 * <strong class="example">Example 2:
 *
 * Input: nums = [19,12,14,6,10,18], k = 3
 * Output: 4788
 * Explanation: To get a score of 4788, we can apply the following operations:
 * - Choose subarray nums[0, ..., 0]. nums[0] is the only element in this subarray. Hence, we multiply the score by nums[0]. The score becomes 1 * 19 = 19.
 * - Choose subarray nums[5, ..., 5]. nums[5] is the only element in this subarray. Hence, we multiply the score by nums[5]. The score becomes 19 * 18 = 342.
 * - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 2, but nums[2] has the smaller index. Hence, we multipy the score by nums[2]. The score becomes 342 * 14 = 4788.
 * It can be proven that 4788 is the highest score one can obtain.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length == n <= 10^5
 *     1 <= nums[i] <= 10^5
 *     1 <= k <= min(n * (n + 1) / 2, 10^9)
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	const PRIMES: [i32; 65] = [
		2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
		97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
		191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
		283, 293, 307, 311, 313,
	];
	pub fn maximum_score(nums: Vec<i32>, mut k: i32) -> i32 {
		let mut bh: std::collections::BinaryHeap<_> = nums.iter().copied().zip(0..).collect();
		let nums: Vec<_> = nums.into_iter().map(|i| (i, Self::prime_cnt(i))).collect();
		let mut ans = 1_i64;
		while let Some((n, i)) = bh.pop() {
			let mut tail = i;
			let score = nums[i].1;
			while tail + 1 < nums.len() && nums[tail + 1].1 <= score {
				tail += 1;
			}
			let mut head = i;
			while head > 0 && nums[head - 1].1 < score {
				head -= 1;
			}
			let len = k.min(((tail + 1 - i) * (i + 1 - head)) as i32);
			k -= len;
			ans = Self::pow(ans, n as i64, len);
			if k == 0 {
				break;
			}
		}
		ans as i32
	}

	fn pow(mut ans: i64, n: i64, mut k: i32) -> i64 {
		let mut cur = n;
		let mo = (1e9 + 7.0) as i64;
		while k > 0 {
			if k % 2 > 0 {
				ans = (ans * cur) % mo;
			}
			cur = (cur * cur) % mo;
			k /= 2;
		}
		ans
	}

	fn prime_cnt(mut n: i32) -> i32 {
		let mut cnt = 0;
		for pr in Self::PRIMES {
			if pr > n {
				break;
			}
			if n % pr == 0 {
				cnt += 1;
				while n % pr == 0 {
					n /= pr;
				}
			}
		}
		cnt + (n > 1) as i32
	}
}

// submission codes end
