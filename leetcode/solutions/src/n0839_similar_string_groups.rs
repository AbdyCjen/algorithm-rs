/**
 * [839] Similar String Groups
 *
 * Two strings X and Y are similar if we can swap two letters (in different positions) of X, so that it equals Y. Also two strings X and Y are similar if they are equal.
 * For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar, but "star" is not similar to "tars", "rats", or "arts".
 * Together, these form two connected groups by similarity: {"tars", "rats", "arts"} and {"star"}.  Notice that "tars" and "arts" are in the same group even though they are not similar.  Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.
 * We are given a list strs of strings where every string in strs is an anagram of every other string in strs. How many groups are there?
 *  
 * <strong class="example">Example 1:
 *
 * Input: strs = ["tars","rats","arts","star"]
 * Output: 2
 *
 * <strong class="example">Example 2:
 *
 * Input: strs = ["omv","ovm"]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= strs.length <= 300
 *     1 <= strs[i].length <= 300
 *     strs[i] consists of lowercase letters only.
 *     All words in strs have the same length and are anagrams of each other.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_similar_groups(strs: Vec<String>) -> i32 {
		fn find(set: &mut [i32], x: i32) -> i32 {
			if set[x as usize] != x {
				set[x as usize] = find(set, set[x as usize]);
			}
			set[x as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}
		let mut set = (0..strs.len() as i32).collect::<Vec<_>>();
		for (s1, i) in strs.iter().zip(0..) {
			for (s2, j) in strs[(i + 1) as usize..].iter().zip(i + 1..) {
				if find(&mut set, i) != find(&mut set, j) && Self::similar(s1, s2) {
					union(&mut set, i, j);
				}
			}
		}
		(0..strs.len() as i32)
			.filter(|&i| find(&mut set, i) == i)
			.count() as i32
	}

	fn similar(s1: &str, s2: &str) -> bool {
		s1.bytes()
			.zip(s2.bytes())
			.filter(|(c1, c2)| c1 != c2)
			.count() <= 2
	}
}

// submission codes end
