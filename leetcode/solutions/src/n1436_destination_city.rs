/**
 * [1436] Destination City
 *
 * You are given the array paths, where paths[i] = [cityAi, cityBi] means there exists a direct path going from cityAi to cityBi. Return the destination city, that is, the city without any path outgoing to another city.
 * It is guaranteed that the graph of paths forms a line without any loop, therefore, there will be exactly one destination city.
 *  
 * <strong class="example">Example 1:
 *
 * Input: paths = [["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]
 * Output: "Sao Paulo"
 * Explanation: Starting at "London" city you will reach "Sao Paulo" city which is the destination city. Your trip consist of: "London" -> "New York" -> "Lima" -> "Sao Paulo".
 *
 * <strong class="example">Example 2:
 *
 * Input: paths = [["B","C"],["D","B"],["C","A"]]
 * Output: "A"
 * Explanation: All possible trips are:
 * "D" -> "B" -> "C" -> "A".
 * "B" -> "C" -> "A".
 * "C" -> "A".
 * "A".
 * Clearly the destination city is "A".
 *
 * <strong class="example">Example 3:
 *
 * Input: paths = [["A","Z"]]
 * Output: "Z"
 *
 *  
 * Constraints:
 *
 *     1 <= paths.length <= 100
 *     paths[i].length == 2
 *     1 <= cityAi.length, cityBi.length <= 10
 *     cityAi != cityBi
 *     All strings consist of lowercase and uppercase English letters and the space character.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn dest_city(paths: Vec<Vec<String>>) -> String {
		let mut cnts = std::collections::HashMap::new();
		for mut p in paths {
			cnts.entry(p.pop().unwrap()).or_insert((0, 0)).1 += 1;
			cnts.entry(p.pop().unwrap()).or_insert((0, 0)).0 += 1;
		}
		cnts.into_iter().find(|v| v.1 .0 == 0).unwrap().0
	}
}

// submission codes end
