/**
 * [2024] Maximize the Confusion of an Exam
 *
 * A teacher is writing a test with n true/false questions, with 'T' denoting true and 'F' denoting false. He wants to confuse the students by maximizing the number of consecutive questions with the same answer (multiple trues or multiple falses in a row).
 * You are given a string answerKey, where answerKey[i] is the original answer to the i^th question. In addition, you are given an integer k, the maximum number of times you may perform the following operation:
 *
 *     Change the answer key for any question to 'T' or 'F' (i.e., set answerKey[i] to 'T' or 'F').
 *
 * Return the maximum number of consecutive 'T's or 'F's in the answer key after performing the operation at most k times.
 *  
 * <strong class="example">Example 1:
 *
 * Input: answerKey = "TTFF", k = 2
 * Output: 4
 * Explanation: We can replace both the 'F's with 'T's to make answerKey = "<u>TTTT</u>".
 * There are four consecutive 'T's.
 *
 * <strong class="example">Example 2:
 *
 * Input: answerKey = "TFFT", k = 1
 * Output: 3
 * Explanation: We can replace the first 'T' with an 'F' to make answerKey = "<u>FFF</u>T".
 * Alternatively, we can replace the second 'T' with an 'F' to make answerKey = "T<u>FFF</u>".
 * In both cases, there are three consecutive 'F's.
 *
 * <strong class="example">Example 3:
 *
 * Input: answerKey = "TTFTTFTT", k = 1
 * Output: 5
 * Explanation: We can replace the first 'F' to make answerKey = "<u>TTTTT</u>FTT"
 * Alternatively, we can replace the second 'F' to make answerKey = "TTF<u>TTTTT</u>".
 * In both cases, there are five consecutive 'T's.
 *
 *  
 * Constraints:
 *
 *     n == answerKey.length
 *     1 <= n <= 5 * 10^4
 *     answerKey[i] is either 'T' or 'F'
 *     1 <= k <= n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
		let (mut ans, row) = (0, answer_key.as_bytes());
		for (c, mut k) in [(b'T', k), (b'F', k)] {
			let mut i = 0;
			for (j, &cc) in (0..).zip(row) {
				k -= (cc != c) as i32;
				while k < 0 {
					k += (row[i as usize] != c) as i32;
					i += 1;
				}
				ans = ans.max(j - i + 1);
			}
		}
		ans
	}
}

// submission codes end
