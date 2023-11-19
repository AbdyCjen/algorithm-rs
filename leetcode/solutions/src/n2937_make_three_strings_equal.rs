/**
 * [2937] Make Three Strings Equal
 *
 * You are given three strings s1, s2, and s3. You have to perform the following operation on these three strings as many times as you want<!-- notionvc: b5178de7-3318-4129-b7d9-726b47e90621 -->.
 * In one operation you can choose one of these three strings such that its length is at least 2<!-- notionvc: 3342ac46-33c8-4010-aacd-e58678ce31ef --> and delete the rightmost character of it.
 * Return the minimum number of operations you need to perform to make the three strings equal if there is a way to make them equal, otherwise, return -1.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s1 = "abc", s2 = "abb", s3 = "ab"
 * Output: 2
 * Explanation: Performing operations on s1 and s2 once will lead to three equal strings.
 * It can be shown that there is no way to make them equal with less than two operations.
 * <strong class="example">Example 2:
 *
 * Input: s1 = "dac", s2 = "bac", s3 = "cac"
 * Output: -1
 * Explanation: Because the leftmost letters<!-- notionvc: 47239f7c-eec1-49f8-af79-c206ec88cb07 --> of s1 and s2 are not equal, they could not be equal after any number of operations. So the answer is -1.
 *
 *  
 * Constraints:
 *
 *     1 <= s1.length, s2.length, s3.length <= 100
 *     <font face="monospace">s1,</font> <font face="monospace">s2</font><font face="monospace"> and</font> <font face="monospace">s3</font> consist only of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
		if s1 == s2 && s2 == s3 {
			return 0;
		}
		let tot = s1.len() + s2.len() + s3.len();
		let mut pref = 0;
		for ((c1, c2), c3) in s1.bytes().zip(s2.bytes()).zip(s3.bytes()) {
			if c1 == c2 && c2 == c3 {
				pref += 1;
			} else {
				break;
			}
		}
		if pref >= 1 {
			tot as i32 - pref * 3
		} else {
			-1
		}
	}
}

// submission codes end
