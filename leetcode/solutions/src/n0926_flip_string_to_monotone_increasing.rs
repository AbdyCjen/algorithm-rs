/**
 * [926] Flip String to Monotone Increasing
 *
 * A binary string is monotone increasing if it consists of some number of 0's (possibly none), followed by some number of 1's (also possibly none).
 * You are given a binary string s. You can flip s[i] changing it from 0 to 1 or from 1 to 0.
 * Return the minimum number of flips to make s monotone increasing.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "00110"
 * Output: 1
 * Explanation: We flip the last digit to get 00111.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "010110"
 * Output: 2
 * Explanation: We flip to get 011111, or alternatively 000111.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "00011000"
 * Output: 2
 * Explanation: We flip to get 00000000.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_flips_mono_incr(s: String) -> i32 {
		let (mut cnt, mut prv) = (0, s.as_bytes()[0]);
		let mut cnts = vec![];
		for c in s.bytes() {
			if c == prv {
				cnt += 1;
			} else {
				cnts.push(cnt);
				cnt = 1;
				prv = c;
			}
		}
		cnts.push(cnt);
		let n = cnts.len();
		if n == 1 {
			return 0;
		}
		let mut sums = vec![[0; 2]; n];
		let mut prv = [0; 2];
		for (i, (&cnt, sum)) in cnts.iter().zip(&mut sums).enumerate() {
			prv[i % 2] += cnt;
			*sum = prv;
		}
		let zero = (s.as_bytes()[0] != b'0') as usize;
		let total_zero = cnts[zero..].iter().step_by(2).sum::<i32>();
		let [a, b] = *sums.last().unwrap();
		sums.windows(2)
			.fold(i32::MAX, |acc, win| {
				acc.min(win[0][zero ^ 1] + total_zero - win[1][zero])
			})
			.min(a)
			.min(b)
	}
	/* from someone else...
	  pub fn min_flips_mono_incr(s: String) -> i32 {
		  s.bytes().fold((0,0), |(ones, x), c| match c {
		   b'0' => (ones, ones.min(x + 1)),
		   _ => (ones + 1, x),
	   })
	  }
	*/
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_926() {
		assert_eq!(Solution::min_flips_mono_incr("00110".to_owned()), 1);
		assert_eq!(Solution::min_flips_mono_incr("010110".to_owned()), 2);
		assert_eq!(Solution::min_flips_mono_incr("00011000".to_owned()), 2);
	}
}
