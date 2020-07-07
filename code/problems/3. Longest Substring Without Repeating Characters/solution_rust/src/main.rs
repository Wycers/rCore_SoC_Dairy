/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/

// submission codes start here

use std::cmp::max;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut pos: [i32; 128] = [-1; 128];
        let mut ans = 0;
        let mut l = 0;

        for (r, c) in s.chars().enumerate() {
            let (r, c) = (r as i32, c as usize);
            if pos[c] != -1 {
                l = max(l, pos[c] + 1);
            }
            pos[c] = r;
            ans = max(ans, r - l + 1);
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}

fn main() {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name);
    Solution::length_of_longest_substring(name);
}
