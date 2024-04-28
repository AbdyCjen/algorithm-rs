/**
 * [126] Word Ladder II
 *
 * A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
 *
 *     Every adjacent pair of words differs by a single letter.
 *     Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
 *     sk == endWord
 *
 * Given two words, beginWord and endWord, and a dictionary wordList, return all the shortest transformation sequences from beginWord to endWord, or an empty list if no such sequence exists. Each sequence should be returned as a list of the words [beginWord, s1, s2, ..., sk].
 *  
 * <strong class="example">Example 1:
 *
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
 * Output: [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
 * Explanation: There are 2 shortest transformation sequences:
 * "hit" -> "hot" -> "dot" -> "dog" -> "cog"
 * "hit" -> "hot" -> "lot" -> "log" -> "cog"
 *
 * <strong class="example">Example 2:
 *
 * Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
 * Output: []
 * Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
 *
 *  
 * Constraints:
 *
 *     1 <= beginWord.length <= 5
 *     endWord.length == beginWord.length
 *     1 <= wordList.length <= 500
 *     wordList[i].length == beginWord.length
 *     beginWord, endWord, and wordList[i] consist of lowercase English letters.
 *     beginWord != endWord
 *     All the words in wordList are unique.
 *     The sum of all shortest transformation sequences does not exceed 10^5.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	fn dfs(
		cur: usize,
		tar: usize,
		st: &mut Vec<usize>,
		neis: &[Vec<usize>],
		cb: &mut impl FnMut(&[usize]),
	) {
		if cur == tar {
			cb(st);
			//ans.push(st.iter().rev().map(|i| list[*i as usize].clone()).collect());
			return;
		}
		for &ne in &neis[cur] {
			st.push(ne);
			Self::dfs(ne, tar, st, neis, cb);
			st.pop();
		}
	}
	pub fn find_ladders(begin: String, end: String, mut list: Vec<String>) -> Vec<Vec<String>> {
		let Some(end) = list.iter().position(|s| *s == end) else {
			return vec![];
		};
		let begin = match list.iter().position(|s| *s == begin) {
			Some(i) => i,
			_ => {
				list.push(begin.clone());
				list.len() - 1
			}
		};
		let mut neis = std::collections::HashMap::new();
		for (i, s) in (0..).zip(&list) {
			for j in 0..s.len() {
				neis.entry((&s[..j], &s[j + 1..]))
					.or_insert_with(Vec::new)
					.push(i);
			}
		}
		let mut st = std::collections::BTreeSet::new();
		st.insert(begin);
		let mut vis = vec![false; list.len()];
		vis[begin] = true;
		let mut prv = vec![vec![]; list.len()];
		while !st.is_empty() {
			for i in std::mem::take(&mut st) {
				if i == end {
					let mut ans = vec![];
					let mut cb = |st: &[usize]| {
						ans.push(st.iter().rev().map(|i| list[*i].clone()).collect())
					};
					Self::dfs(end, begin, &mut vec![end], &prv, &mut cb);
					return ans;
				}
				let s = &list[i];
				for j in 0..s.len() {
					for &ne in neis.get(&(&s[..j], &s[j + 1..])).into_iter().flatten() {
						if !vis[ne] {
							prv[ne].push(i);
							st.insert(ne);
						}
					}
				}
			}
			st.iter().for_each(|i| vis[*i] = true);
		}
		vec![]
	}
}

// submission codes end
