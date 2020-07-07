/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order and each of their nodes
 * contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 */
pub struct Solution {}

// problem: https://leetcode-cn.com/problems/two-sum/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// submission codes start here

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut p = &mut res;

        let (mut l1, mut l2) = (l1, l2);

        let mut c = 0;

        loop {
            if let Some(v1) = l1 {
                c += v1.val;
                l1 = v1.next;
            }
            if let Some(v2) = l2 {
                c += v2.val;
                l2 = v2.next;
            }

            let flag = l1 == None && l2 == None;

            if let Some(i) = p {
                i.val = c % 10;
                c /= 10;
                i.next = if flag && c == 0 {
                    None
                } else {
                    Some(Box::new(ListNode::new(c)))
                };
                p = &mut i.next;
            }

            if flag {
                break;
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
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn main() {}
