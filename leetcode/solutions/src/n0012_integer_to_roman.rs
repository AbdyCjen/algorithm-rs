/**
 * [12] Integer to Roman
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 *     I can be placed before V (5) and X (10) to make 4 and 9.
 *     X can be placed before L (50) and C (100) to make 40 and 90.
 *     C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 * Given an integer, convert it to a roman numeral.
 *  
 * <strong class="example">Example 1:
 *
 * Input: num = 3
 * Output: "III"
 * Explanation: 3 is represented as 3 ones.
 *
 * <strong class="example">Example 2:
 *
 * Input: num = 58
 * Output: "LVIII"
 * Explanation: L = 50, V = 5, III = 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: num = 1994
 * Output: "MCMXCIV"
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 *  
 * Constraints:
 *
 *     1 <= num <= 3999
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn int_to_roman(mut num: i32) -> String {
		const NOT: &[char] = &['I', 'V', 'X', 'L', 'C', 'D', 'M'];
		let mut i = 0;
		let mut ans = vec![];
		while num > 0 {
			let s: String = match num % 10 {
				n @ 0..=3 => vec![NOT[i]; n as usize].iter().collect(),
				4 => [NOT[i], NOT[i + 1]].iter().collect(),
				n @ 5..=8 => [NOT[i + 1]]
					.iter()
					.chain(std::iter::repeat(&NOT[i]).take(n as usize - 5))
					.collect(),
				// 9
				_ => [NOT[i], NOT[i + 2]].iter().collect(),
			};
			ans.push(s);
			i += 2;
			num /= 10;
		}
		ans.into_iter().rev().collect()
	}
}

// submission codes end
