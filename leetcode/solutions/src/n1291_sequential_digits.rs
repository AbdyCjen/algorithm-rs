/**
 * [1291] Sequential Digits
 *
 * An integer has sequential digits if and only if each digit in the number is one more than the previous digit.
 * Return a sorted list of all the integers in the range [low, high] inclusive that have sequential digits.
 *  
 * <strong class="example">Example 1:
 * Input: low = 100, high = 300
 * Output: [123,234]
 * <strong class="example">Example 2:
 * Input: low = 1000, high = 13000
 * Output: [1234,2345,3456,4567,5678,6789,12345]
 *  
 * Constraints:
 *
 *     10 <= low <= high <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
		let (mut base, mut step, mut n) = (12, 11, 2);
		std::iter::successors(Some(base), |i| {
			Some(if (i + step) % 10 != 0 {
				i + step
			} else {
				n += 1;
				base = base * 10 + n;
				step = step * 10 + 1;
				base
			})
		})
		.filter(|i| *i >= low)
		.take_while(|i| *i <= high)
		.collect()
	}
}

// submission codes end
