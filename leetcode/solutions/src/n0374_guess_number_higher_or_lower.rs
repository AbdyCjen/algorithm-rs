use std::{cmp::Ordering, sync::atomic::AtomicI32};

/**
 * [374] Guess Number Higher or Lower
 *
 * We are playing the Guess Game. The game is as follows:
 * I pick a number from 1 to n. You have to guess which number I picked.
 * Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
 * You call a pre-defined API int guess(int num), which returns three possible results:
 *
 *     -1: Your guess is higher than the number I picked (i.e. num > pick).
 *     1: Your guess is lower than the number I picked (i.e. num < pick).
 *     0: your guess is equal to the number I picked (i.e. num == pick).
 *
 * Return the number that I picked.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 10, pick = 6
 * Output: 6
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1, pick = 1
 * Output: 1
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 2, pick = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 2^31 - 1
 *     1 <= pick <= n
 *
 */
pub struct Solution {}

static N: AtomicI32 = AtomicI32::new(0);

unsafe fn guess(n: i32) -> i32 {
	match N.load(std::sync::atomic::Ordering::Relaxed).cmp(&n) {
		Ordering::Less => -1,
		Ordering::Equal => 0,
		Ordering::Greater => 1,
	}
}
// submission codes start here

#[allow(non_snake_case)]
impl Solution {
	unsafe fn guessNumber(n: i32) -> i32 {
		let (mut l, mut r) = (1, n);
		while l < r {
			let m = l + (r - l) / 2;
			match guess(m) {
				1 => l = m + 1,
				_ => r = m,
			}
		}
		l
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_374() {
		unsafe fn test(ans: i32) {
			N.store(ans, std::sync::atomic::Ordering::Relaxed);
			assert_eq!(Solution::guessNumber(ans + 20), ans);
		}
		unsafe {
			for i in 1..100 {
				test(i);
			}
		};
	}
}
