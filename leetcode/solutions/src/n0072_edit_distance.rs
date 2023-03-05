/**
 * [72] Edit Distance
 *
 * Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
 *
 * You have the following 3 operations permitted on a word:
 *
 * <ol>
 * Insert a character
 * Delete a character
 * Replace a character
 * </ol>
 *
 * Example 1:
 *
 *
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation:
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 *
 *
 * Example 2:
 *
 *
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation:
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 *
 *
 */
pub struct Solution {}

// submission codes start here
/*
 * DP, 使得mat[i][j]表示使word1[0..i-1]转换成word2[0..j-1] 所需要的最少操作数量
 * 显然地, 有mat[i][0] = i, mat[0][j] = j, 目标的返回值是mat[word1.len()][word2.len()]
 * 如果已知mat[i-1][j-1], mat[i-1][j], mat[i][j-1],
 * */
impl Solution {
	pub fn min_distance(word1: String, word2: String) -> i32 {
		if word1.is_empty() || word2.is_empty() {
			return (word1.len() + word2.len()) as i32;
		}

		let mut cache = (0..=word2.len() as i32).collect::<Vec<_>>();
		for (c1, i) in word1.bytes().zip(0..) {
			let mut prev = cache[0];
			cache[0] = i + 1;
			for (j, c2) in word2.bytes().enumerate() {
				let temp = cache[j + 1];
				cache[j + 1] = if c1 == c2 {
					prev
				} else {
					prev.min(temp).min(cache[j]) + 1
				};
				prev = temp;
			}
		}
		cache[word2.len()]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_72() {
		assert_eq!(
			Solution::min_distance("sea".to_owned(), "ate".to_owned()),
			3
		);
		assert_eq!(
			Solution::min_distance("horse".to_owned(), "ros".to_owned()),
			3
		);
		assert_eq!(
			Solution::min_distance("intention".to_owned(), "execution".to_owned()),
			5
		);
		assert_eq!(
			Solution::min_distance("abdfdgsafdsafda".to_owned(), "abdfdgsafdsafda".to_owned()),
			0
		);
		assert_eq!(Solution::min_distance("".to_owned(), "".to_owned()), 0);
	}
}
