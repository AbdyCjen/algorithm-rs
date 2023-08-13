/**
 * [332] Reconstruct Itinerary
 *
 * You are given a list of airline tickets where tickets[i] = [fromi, toi] represent the departure and the arrival airports of one flight. Reconstruct the itinerary in order and return it.
 * All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK". If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.
 *
 *     For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
 *
 * You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/itinerary1-graph.jpg" style="width: 382px; height: 222px;" />
 * Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
 * Output: ["JFK","MUC","LHR","SFO","SJC"]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/itinerary2-graph.jpg" style="width: 222px; height: 230px;" />
 * Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
 * Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
 * Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"] but it is larger in lexical order.
 *
 *  
 * Constraints:
 *
 *     1 <= tickets.length <= 300
 *     tickets[i].length == 2
 *     fromi.length == 3
 *     toi.length == 3
 *     fromi and toi consist of uppercase English letters.
 *     fromi != toi
 *
 */
pub struct Solution {}

// submission codes start here

// Euler path
use std::collections::{HashMap, VecDeque};
impl Solution {
	pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
		let mut map = HashMap::<_, Vec<_>>::new();
		for mut tik in tickets {
			if let (Some(to), Some(from)) = (tik.pop(), tik.pop()) {
				map.entry(from).or_default().push(to);
			}
		}
		map.values_mut()
			.for_each(|v| v.sort_unstable_by(|a, b| b.cmp(a)));
		let mut ans = VecDeque::new();
		Self::solve("JFK".to_owned(), &mut ans, &mut map);
		ans.into()
	}
	fn solve<'a>(cur: String, st: &mut VecDeque<String>, map: &mut HashMap<String, Vec<String>>) {
		while let Some(next) = map.get_mut(&cur).and_then(|v| v.pop()) {
			Self::solve(next, st, map);
		}
		st.push_front(cur);
	}
}

// submission codes end
