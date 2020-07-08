/**
 * [6] ZigZag Conversion
 *
 * The string <code>"PAYPALISHIRING"</code> is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: <code>"PAHNAPLSIIGYIR"</code>
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/

// submission codes start here

impl Solution {
    pub fn convert(s: String, n: i32) -> String {
        if n == 1 {
            return s;
        }

        let s: Vec<char> = s.chars().collect();
        let len = s.len() as i32;
        let step = (n - 1) << 1;
        let mut res = String::new();

        for i in 0..n {
            let flag = i == 0 || i == n - 1;
            for j in (i..len).step_by(step as usize) {
                res.push(s[j as usize]);
                if flag {
                    continue;
                }
                let k = j + step - (i << 1);
                if k < len {
                    res.push(s[k as usize]);
                }
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
        assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
    }
}

fn main() {}
