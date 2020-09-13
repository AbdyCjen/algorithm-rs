/**
 * [68] Text Justification
 *
 * Given an array of words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.
 *
 * You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.
 *
 * Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line do not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
 *
 * For the last line of text, it should be left justified and no extra space is inserted between words.
 *
 * Note:
 *
 *
 *     A word is defined as a character sequence consisting of non-space characters only.
 *     Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
 *     The input array words contains at least one word.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * words = ["This", "is", "an", "example", "of", "text", "justification."]
 * maxWidth = 16
 * Output:
 * [
 *    "This    is    an",
 *    "example  of text",
 *    "justification.  "
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input:
 * words = ["What","must","be","acknowledgment","shall","be"]
 * maxWidth = 16
 * Output:
 * [
 *   "What   must   be",
 *   "acknowledgment  ",
 *   "shall be        "
 * ]
 * Explanation: Note that the last line is "shall be    " instead of "shall     be",
 *              because the last line must be left-justified instead of fully-justified.
 *              Note that the second line is also left-justified becase it contains only one word.
 *
 *
 * Example 3:
 *
 *
 * Input:
 * words = ["Science","is","what","we","understand","well","enough","to","explain",
 *          "to","a","computer.","Art","is","everything","else","we","do"]
 * maxWidth = 20
 * Output:
 * [
 *   "science  is  what we",
 *   "understand      well",
 *   "enough to explain to",
 *   "a  computer.  art is",
 *   "everything  else  we",
 *   "do                  "
 * ]
 *
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
		let mut res = Vec::new();
		let mut cache = Vec::new();
		let mut wlen = 0;
		for word in words.into_iter() {
			if wlen + cache.len() + word.len() > max_width as usize {
				res.push(Solution::line_compact(&mut cache, wlen, max_width));
				wlen = word.len();
				cache.push(word);
			} else {
				wlen += word.len();
				cache.push(word);
			}
		}
		res.push(Solution::last_line_compact(&mut cache, wlen, max_width));
		res
	}

	fn line_compact(cache: &mut Vec<String>, wlen: usize, max_width: i32) -> String {
		let spcs = max_width as usize - wlen;
		let avg_spc = spcs / std::cmp::max(1, cache.len() - 1);
		let ext_spc = spcs - avg_spc * std::cmp::max(1, cache.len() - 1);
		let mut res = String::new();
		for (i, word) in cache.iter().enumerate() {
			res.push_str(word);
			if i < cache.len() - 1 || cache.len() == 1 {
				res.push_str(&" ".repeat(avg_spc));
				if i < ext_spc {
					res.push(' ');
				}
			}
		}
		cache.clear();
		res
	}

	fn last_line_compact(cache: &mut Vec<String>, wlen: usize, max_width: i32) -> String {
		let spcs = max_width as usize - wlen;
		let last_spc = spcs - cache.len() + 1;
		let mut res = String::new();
		for (i, word) in cache.iter().enumerate() {
			res.push_str(word);
			if i < cache.len() - 1 {
				res.push(' ');
			}
		}
		res.push_str(&" ".repeat(last_spc));
		res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_68() {
		assert_eq!(
			Solution::full_justify(
				vec_string![
					"This",
					"is",
					"an",
					"example",
					"of",
					"text",
					"justification."
				],
				16
			),
			vec_string!["This    is    an", "example  of text", "justification.  "]
		);
		assert_eq!(
			Solution::full_justify(
				vec_string!["What", "must", "be", "acknowledgment", "shall", "be"],
				16
			),
			vec_string!["What   must   be", "acknowledgment  ", "shall be        "]
		);
		assert_eq!(
			Solution::full_justify(
				vec_string![
					"Science",
					"is",
					"what",
					"we",
					"understand",
					"well",
					"enough",
					"to",
					"explain",
					"to",
					"a",
					"computer.",
					"Art",
					"is",
					"everything",
					"else",
					"we",
					"do"
				],
				20
			),
			vec_string![
				"Science  is  what we",
				"understand      well",
				"enough to explain to",
				"a  computer.  Art is",
				"everything  else  we",
				"do                  "
			]
		)
	}
}
