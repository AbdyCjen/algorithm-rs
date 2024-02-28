/**
 * [2977] Minimum Cost to Convert String II
 *
 * You are given two 0-indexed strings source and target, both of length n and consisting of lowercase English characters. You are also given two 0-indexed string arrays original and changed, and an integer array cost, where cost[i] represents the cost of converting the string original[i] to the string changed[i].
 * You start with the string source. In one operation, you can pick a substring x from the string, and change it to y at a cost of z if there exists any index j such that cost[j] == z, original[j] == x, and changed[j] == y. You are allowed to do any number of operations, but any pair of operations must satisfy either of these two conditions:
 *
 *     The substrings picked in the operations are source[a..b] and source[c..d] with either b < c or d < a. In other words, the indices picked in both operations are disjoint.
 *     The substrings picked in the operations are source[a..b] and source[c..d] with a == c and b == d. In other words, the indices picked in both operations are identical.
 *
 * Return the minimum cost to convert the string source to the string target using any number of operations. If it is impossible to convert source to target, return -1.
 * Note that there may exist indices i, j such that original[j] == original[i] and changed[j] == changed[i].
 *  
 * <strong class="example">Example 1:
 *
 * Input: source = "abcd", target = "acbe", original = ["a","b","c","c","e","d"], changed = ["b","c","b","e","b","e"], cost = [2,5,5,1,2,20]
 * Output: 28
 * Explanation: To convert "abcd" to "acbe", do the following operations:
 * - Change substring source[1..1] from "b" to "c" at a cost of 5.
 * - Change substring source[2..2] from "c" to "e" at a cost of 1.
 * - Change substring source[2..2] from "e" to "b" at a cost of 2.
 * - Change substring source[3..3] from "d" to "e" at a cost of 20.
 * The total cost incurred is 5 + 1 + 2 + 20 = 28.
 * It can be shown that this is the minimum possible cost.
 *
 * <strong class="example">Example 2:
 *
 * Input: source = "abcdefgh", target = "acdeeghh", original = ["bcd","fgh","thh"], changed = ["cde","thh","ghh"], cost = [1,3,5]
 * Output: 9
 * Explanation: To convert "abcdefgh" to "acdeeghh", do the following operations:
 * - Change substring source[1..3] from "bcd" to "cde" at a cost of 1.
 * - Change substring source[5..7] from "fgh" to "thh" at a cost of 3. We can do this operation because indices [5,7] are disjoint with indices picked in the first operation.
 * - Change substring source[5..7] from "thh" to "ghh" at a cost of 5. We can do this operation because indices [5,7] are disjoint with indices picked in the first operation, and identical with indices picked in the second operation.
 * The total cost incurred is 1 + 3 + 5 = 9.
 * It can be shown that this is the minimum possible cost.
 *
 * <strong class="example">Example 3:
 *
 * Input: source = "abcdefgh", target = "addddddd", original = ["bcd","defgh"], changed = ["ddd","ddddd"], cost = [100,1578]
 * Output: -1
 * Explanation: It is impossible to convert "abcdefgh" to "addddddd".
 * If you select substring source[1..3] as the first operation to change "abcdefgh" to "adddefgh", you cannot select substring source[3..7] as the second operation because it has a common index, 3, with the first operation.
 * If you select substring source[3..7] as the first operation to change "abcdefgh" to "abcddddd", you cannot select substring source[1..3] as the second operation because it has a common index, 3, with the first operation.
 *
 *  
 * Constraints:
 *
 *     1 <= source.length == target.length <= 1000
 *     source, target consist only of lowercase English characters.
 *     1 <= cost.length == original.length == changed.length <= 100
 *     1 <= original[i].length == changed[i].length <= source.length
 *     original[i], changed[i] consist only of lowercase English characters.
 *     original[i] != changed[i]
 *     1 <= cost[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

struct Trie {
	child: Vec<Self>,
	c: u8,
	end: Option<i32>,
}

impl Trie {
	fn new(c: u8) -> Self {
		Self {
			child: vec![],
			c,
			end: None,
		}
	}
	fn insert(&mut self, s: &str, end: i32) -> bool {
		let mut cur = self;
		for c in s.bytes() {
			let idx = match cur.child.binary_search_by_key(&c, |t| t.c) {
				Ok(idx) => idx,
				Err(idx) => {
					cur.child.insert(idx, Self::new(c));
					idx
				}
			};
			cur = &mut cur.child[idx];
		}
		match &mut cur.end {
			Some(_) => false,
			e => {
				*e = Some(end);
				true
			}
		}
	}

	fn get(&self, s: &str) -> Option<i32> {
		let mut cur = self;
		for c in s.bytes() {
			let idx = cur.child.binary_search_by_key(&c, |v| v.c).ok()?;
			cur = &cur.child[idx];
		}
		cur.end.clone()
	}

	fn next(&self, c: u8) -> Option<&Self> {
		let idx = self.child.binary_search_by_key(&c, |t| t.c).ok()?;
		Some(&self.child[idx])
	}
}
impl Solution {
	pub fn minimum_cost(
		source: String,
		target: String,
		original: Vec<String>,
		changed: Vec<String>,
		cost: Vec<i32>,
	) -> i64 {
		let mut max_len = 0;
		let mut len = 0;
		let mut smap = Trie::new(0);
		let mut groups = std::collections::HashMap::new();
		for s in original.iter().chain(&changed) {
			if smap.insert(s, len) {
				groups
					.entry(s.len())
					.or_insert_with(Vec::new)
					.push(len as usize);
				len += 1;
			}
			max_len = max_len.max(s.len());
		}
		let mut graph = vec![vec![i64::MAX; len as usize]; len as usize];
		for i in 0..graph.len() {
			graph[i][i] = 0;
		}
		for ((fs, ts), c) in original.iter().zip(&changed).zip(cost) {
			let f = smap.get(fs.as_str()).unwrap() as usize;
			let t = smap.get(ts.as_str()).unwrap() as usize;
			let cost = &mut graph[f][t];
			*cost = (c as i64).min(*cost);
		}
		Self::floyd(
			&mut graph,
			groups.into_values().collect::<Vec<_>>().as_slice(),
		);
		match Self::solve(
			&source,
			&target,
			&smap,
			&graph,
			max_len,
			&mut vec![-1; source.len() + 1],
		) {
			i64::MAX => -1,
			n => n,
		}
	}
	fn solve(
		sou: &str,
		tar: &str,
		smap: &Trie,
		graph: &[Vec<i64>],
		max_len: usize,
		cache: &mut [i64],
	) -> i64 {
		if cache[sou.len()] >= 0 {
			return cache[sou.len()];
		}
		let mut ans = i64::MAX;
		if sou == tar {
			ans = 0;
		} else {
			let mut all_eq = true;
			let mut tr_s = Some(smap);
			let mut tr_t = Some(smap);
			for ((i, s), t) in (1..).zip(sou.bytes()).zip(tar.bytes()).take(max_len) {
				all_eq &= s == t;
				tr_s = tr_s.and_then(|tr| Trie::next(tr, s));
				tr_t = tr_t.and_then(|tr| Trie::next(tr, t));
				if all_eq {
					ans = ans.min(Self::solve(
						&sou[i..],
						&tar[i..],
						smap,
						graph,
						max_len,
						cache,
					));
				} else if let (Some(ts), Some(tt)) = (tr_s, tr_t) {
					if let (Some(f), Some(t)) = (ts.end, tt.end) {
						ans = ans.min(graph[f as usize][t as usize].saturating_add(Self::solve(
							&sou[i..],
							&tar[i..],
							smap,
							graph,
							max_len,
							cache,
						)));
					}
				}
			}
		}
		cache[sou.len()] = ans;
		ans
	}
	fn floyd(graph: &mut [Vec<i64>], groups: &[Vec<usize>]) {
		for g in groups {
			for &k in g {
				for &x in g {
					for &y in g {
						graph[x][y] = graph[x][y].min(graph[x][k].saturating_add(graph[k][y]));
					}
				}
			}
		}
	}
}

// submission codes end
