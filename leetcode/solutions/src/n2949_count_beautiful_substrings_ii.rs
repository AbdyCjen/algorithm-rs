/**
 * [2949] Count Beautiful Substrings II
 *
 * You are given a string s and a positive integer k.
 * Let vowels and consonants be the number of vowels and consonants in a string.
 * A string is beautiful if:
 *
 *     vowels == consonants.
 *     (vowels * consonants) % k == 0, in other terms the multiplication of vowels and consonants is divisible by k.
 *
 * Return the number of non-empty beautiful substrings in the given string s.
 * A substring is a contiguous sequence of characters in a string.
 * Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.
 * Consonant letters in English are every letter except vowels.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "baeyh", k = 2
 * Output: 2
 * Explanation: There are 2 beautiful substrings in the given string.
 * - Substring "b<u>aeyh</u>", vowels = 2 (["a",e"]), consonants = 2 (["y","h"]).
 * You can see that string "aeyh" is beautiful as vowels == consonants and vowels * consonants % k == 0.
 * - Substring "<u>baey</u>h", vowels = 2 (["a",e"]), consonants = 2 (["b","y"]).
 * You can see that string "baey" is beautiful as vowels == consonants and vowels * consonants % k == 0.
 * It can be shown that there are only 2 beautiful substrings in the given string.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abba", k = 1
 * Output: 3
 * Explanation: There are 3 beautiful substrings in the given string.
 * - Substring "<u>ab</u>ba", vowels = 1 (["a"]), consonants = 1 (["b"]).
 * - Substring "ab<u>ba</u>", vowels = 1 (["a"]), consonants = 1 (["b"]).
 * - Substring "<u>abba</u>", vowels = 2 (["a","a"]), consonants = 2 (["b","b"]).
 * It can be shown that there are only 3 beautiful substrings in the given string.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "bcdf", k = 1
 * Output: 0
 * Explanation: There are no beautiful substrings in the given string.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 5 * 10^4
 *     1 <= k <= 1000
 *     s consists of only English lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	const V: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
	pub fn beautiful_substrings(s: String, k: i32) -> i32 {
		use std::collections::*;
		let s = s.into_bytes();
		let mut sums = HashMap::new();
		let step = Self::step(k);
		let mut ans = 0;
		sums.entry(0).or_insert_with(HashMap::new).insert(0, 1);
		let mut sum = 0;
		for (i, c) in (1..).zip(&s) {
			if Self::V.contains(c) {
				sum += 1;
			} else {
				sum -= 1;
			}
			//TODO: classify idxs with (idxs mod step(k))
			let e = sums
				.entry(sum)
				.or_insert_with(HashMap::new)
				.entry(i % step)
				.or_insert(0);
			ans += *e;
			*e += 1;
		}
		ans
	}

	#[inline]
	fn check(len: i32, k: i32) -> bool { len % 2 == 0 && (len / 2) * (len / 2) % k == 0 }

	// return smallest int`len`  satisfy: (len / 2) ^ 2 % k == 0
	fn step(mut k: i32) -> i32 {
		let mut a = 2;
		for i in 2.. {
			if i > k {
				break;
			}
			let mut c = 0;
			while k % i == 0 {
				k /= i;
				c += 1;
			}
			for _ in 0..(c + 1) / 2 {
				a *= i;
			}
		}
		a
	}
}

// submission codes end
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_2949() {
		assert_eq!(Solution::beautiful_substrings("baeyh".to_owned(), 2), 2);
		assert_eq!(Solution::beautiful_substrings("abba".to_owned(), 1), 3);
	}
}
