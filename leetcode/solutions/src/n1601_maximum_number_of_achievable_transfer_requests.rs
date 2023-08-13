/**
 * [1601] Maximum Number of Achievable Transfer Requests
 *
 * We have n buildings numbered from 0 to n - 1. Each building has a number of employees. It's transfer season, and some employees want to change the building they reside in.
 * You are given an array requests where requests[i] = [fromi, toi] represents an employee's request to transfer from building fromi to building toi.
 * All buildings are full, so a list of requests is achievable only if for each building, the net change in employee transfers is zero. This means the number of employees leaving is equal to the number of employees moving in. For example if n = 3 and two employees are leaving building 0, one is leaving building 1, and one is leaving building 2, there should be two employees moving to building 0, one employee moving to building 1, and one employee moving to building 2.
 * Return the maximum number of achievable requests.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/10/move1.jpg" style="width: 600px; height: 406px;" />
 * Input: n = 5, requests = [[0,1],[1,0],[0,1],[1,2],[2,0],[3,4]]
 * Output: 5
 * Explantion: Let's see the requests:
 * From building 0 we have employees x and y and both want to move to building 1.
 * From building 1 we have employees a and b and they want to move to buildings 2 and 0 respectively.
 * From building 2 we have employee z and they want to move to building 0.
 * From building 3 we have employee c and they want to move to building 4.
 * From building 4 we don't have any requests.
 * We can achieve the requests of users x and b by swapping their places.
 * We can achieve the requests of users y, a and z by swapping the places in the 3 buildings.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/10/move2.jpg" style="width: 450px; height: 327px;" />
 * Input: n = 3, requests = [[0,0],[1,2],[2,1]]
 * Output: 3
 * Explantion: Let's see the requests:
 * From building 0 we have employee x and they want to stay in the same building 0.
 * From building 1 we have employee y and they want to move to building 2.
 * From building 2 we have employee z and they want to move to building 1.
 * We can achieve all the requests.
 * <strong class="example">Example 3:
 *
 * Input: n = 4, requests = [[0,3],[3,1],[1,2],[2,0]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 20
 *     1 <= requests.length <= 16
 *     requests[i].length == 2
 *     0 <= fromi, toi < n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
		Self::solve(&requests, &mut vec![0; n as usize], 0, n)
	}
	fn solve(requests: &[Vec<i32>], delta: &mut [i32], cnt: i32, mut zero: i32) -> i32 {
		match requests {
			[] if zero == delta.len() as i32 => cnt,
			[] => 0,
			[req, requests @ ..] => {
				let mut ans = Self::solve(requests, delta, cnt, zero);
				let (i, j) = (req[0] as usize, req[1] as usize);
				zero -= (delta[i] == 0) as i32 + (delta[j] == 0) as i32;
				delta[i] += 1;
				delta[j] -= 1;
				zero += (delta[i] == 0) as i32 + (delta[j] == 0) as i32;
				ans = ans.max(Self::solve(requests, delta, cnt + 1, zero));
				delta[i] -= 1;
				delta[j] += 1;
				ans
			}
		}
	}
	pub fn maximum_requests1(_: i32, requests: Vec<Vec<i32>>) -> i32 {
		(0_i32..(1 << requests.len()))
			.filter(|i| {
				let mut cnt = [0; 20];
				for (j, req) in (0..).zip(&requests) {
					if ((1 << j) & i) > 0 {
						cnt[req[0] as usize] -= 1;
						cnt[req[1] as usize] += 1;
					}
				}
				cnt == [0; 20]
			})
			.map(|i| i.count_ones() as i32)
			.max()
			.unwrap_or(0)
	}
}

// submission codes end
