/**
 * [2942] Find Words Containing Character
 *
 * You are given a 0-indexed array of strings words and a character x.
 * Return an array of indices representing the words that contain the character x.
 * Note that the returned array may be in any order.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["leet","code"], x = "e"
 * Output: [0,1]
 * Explanation: "e" occurs in both words: "l<u>ee</u>t", and "cod<u>e</u>". Hence, we return indices 0 and 1.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words = ["abc","bcd","aaaa","cbc"], x = "a"
 * Output: [0,2]
 * Explanation: "a" occurs in "<u>a</u>bc", and "<u>aaaa</u>". Hence, we return indices 0 and 2.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: words = ["abc","bcd","aaaa","cbc"], x = "z"
 * Output: []
 * Explanation: "z" does not occur in any of the words. Hence, we return an empty array.
 * 
 *  
 * Constraints:
 * 
 *     1 <= words.length <= 50
 *     1 <= words[i].length <= 50
 *     x is a lowercase English letter.
 *     words[i] consists only of lowercase English letters.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
		let mut ans = vec![];
		for (i, w) in (0..).zip(words) {
			if w.contains(x) {
				ans.push(i);
			}
		}
		ans
    }
}

// submission codes end
