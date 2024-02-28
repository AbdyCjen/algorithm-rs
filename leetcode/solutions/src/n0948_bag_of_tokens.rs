/**
 * [948] Bag of Tokens
 *
 * You start with an initial power of power, an initial score of 0, and a bag of tokens given as an integer array tokens, where each tokens[i] donates the value of tokeni.
 * Your goal is to maximize the total score by strategically playing these tokens. In one move, you can play an unplayed token in one of the two ways (but not both for the same token):
 *
 *     Face-up: If your current power is at least tokens[i], you may play tokeni, losing tokens[i] power and gaining 1 score.
 *     Face-down: If your current score is at least 1, you may play tokeni, gaining tokens[i] power and losing 1 score.
 *
 * Return the maximum possible score you can achieve after playing any number of tokens.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block" style="
 *     border-color: var(--border-tertiary);
 *     border-left-width: 2px;
 *     color: var(--text-secondary);
 *     font-size: .875rem;
 *     margin-bottom: 1rem;
 *     margin-top: 1rem;
 *     overflow: visible;
 *     padding-left: 1rem;
 * ">
 * Input: <span class="example-io" style="
 *     font-family: Menlo,sans-serif;
 *     font-size: 0.85rem;
 * ">tokens = [100], power = 50</span>
 * Output: <span class="example-io" style="
 *     font-family: Menlo,sans-serif;
 *     font-size: 0.85rem;
 * ">0</span>
 * Explanation: Since your score is 0 initially, you cannot play the token face-down. You also cannot play it face-up since your power (50) is less than tokens[0] (100).
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block" style="
 *     border-color: var(--border-tertiary);
 *     border-left-width: 2px;
 *     color: var(--text-secondary);
 *     font-size: .875rem;
 *     margin-bottom: 1rem;
 *     margin-top: 1rem;
 *     overflow: visible;
 *     padding-left: 1rem;
 * ">
 * Input: <span class="example-io" style="
 *     font-family: Menlo,sans-serif;
 *     font-size: 0.85rem;
 * ">tokens = [200,100], power = 150</span>
 * Output: <span class="example-io" style="
 *     font-family: Menlo,sans-serif;
 *     font-size: 0.85rem;
 * ">1</span>
 * Explanation: Play token1 (100) face-up, reducing your power to 50 and increasing your score to 1.
 * There is no need to play token0, since you cannot play it face-up to add to your score. The maximum score achievable is 1.
 * </div>
 * <strong class="example">Example 3:
 * <div class="example-block" style="
 *     border-color: var(--border-tertiary);
 *     border-left-width: 2px;
 *     color: var(--text-secondary);
 *     font-size: .875rem;
 *     margin-bottom: 1rem;
 *     margin-top: 1rem;
 *     overflow: visible;
 *     padding-left: 1rem;
 * ">
 * Input: <span class="example-io" style="
 *     font-family: Menlo,sans-serif;
 *     font-size: 0.85rem;
 * ">tokens = [100,200,300,400], power = 200</span>
 * Output: <span class="example-io" style="
 *     font-family: Menlo,sans-serif;
 *     font-size: 0.85rem;
 * ">2</span>
 * Explanation: Play the tokens in this order to get a score of 2:
 * <ol>
 *     Play token0 (100) face-up, reducing power to 100 and increasing score to 1.
 *     Play token3 (400) face-down, increasing power to 500 and reducing score to 0.
 *     Play token1 (200) face-up, reducing power to 300 and increasing score to 1.
 *     Play token2 (300) face-up, reducing power to 0 and increasing score to 2.
 * </ol>
 * <span style="color: var(--text-secondary); font-size: 0.875rem;">The maximum score achievable is </span><code style="color: var(--text-secondary); font-size: 0.875rem;">2<span style="color: var(--text-secondary); font-size: 0.875rem;">.</span>
 * </div>
 *  
 * Constraints:
 *
 *     0 <= tokens.length <= 1000
 *     0 <= tokens[i], power < 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
		tokens.sort_unstable();
		let mut score = 0;
		let mut toks = &tokens[..];
		while let [a, rest @ ..] = toks {
			if *a <= power {
				power -= a;
				score += 1;
				toks = rest;
			} else if let [pref @ .., b] = rest {
				if score > 0 {
					power += b - a;
					toks = pref;
				} else {
					break;
				}
			} else {
				break;
			}
		}
		score
	}
}

// submission codes end
