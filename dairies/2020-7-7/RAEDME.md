# rCore_SoC_Dairy

# 2020-7-7

## 回顾

- 实现了一个内核堆
- 用 Rust 刷了 LeetCode #1

## Lab

……

## 刷题

### LeetCode 2. Add Two Numbers

#### cpp version

```cpp
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        if (l1 == NULL && l2 == NULL)
            return NULL;

        ListNode* res = new ListNode(0);
        ListNode* now = res;
        bool flag = true;
        while (l1 != NULL || l2 != NULL)
        {
            if (!flag)
            {
                now->next = new ListNode(0);
                now = now->next;
            }
            now->val += getVal(&l1) + getVal(&l2);
            if (flag = (now->val > 9))
            {
                now->next = new ListNode(1);
                now->val -= 10;
                now = now->next;
            }
        }
        return res;
    }
    inline int getVal(ListNode** node)
    {
        if (*node == NULL)
            return 0;
        int tmp = (*node)->val;
        *node = (*node)->next;
        return tmp;
    }
};
```

#### rust version

果然还是不还写 rust，这个题目竟然写了半小时。。不过也算是对很多知识点扫盲了吧

（噫好像和 cpp 版本不等价

```rust

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

```

### LeetCode 3. Longest Substring Without Repeating Characters

#### cpp version

```cpp
class Solution
{
public:
    int lengthOfLongestSubstring(string s)
    {
        int pos[128];
        for (int i = 0; i < 128; ++i)
            pos[i] = -1;
        int ans = 0, l = 0;
        for (int r = 0, len = s.length(); r < len; ++r)
        {
            int c = s[r];
            if (pos[c] != -1)
                l = max(l, pos[c] + 1);
            pos[c] = r;
            ans = max(ans, r - l + 1);
        }
        return ans;
    }
};

```

#### rust version

学习了一下数组和 for 的使用。

```rust

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

```

### LeetCode 7. Reverse Integer

#### cpp version

```cpp
class Solution {
public:
    int reverse(int x) {
        int res = 0;
        while (x)
        {
            if (abs(res) > INT_MAX / 10)
                return 0;
            res = res * 10 + x % 10;
            x /= 10;
        }
        return res;
    }
};
```

#### rust version

这个题相对简单了一点

```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x: i64 = x as i64;
        let mut res: i64 = 0;
        while x != 0 {
            res = res * 10 + (x % 10);
            x = x / 10;
        }
        if res < -(1 << 31) || res > (1 << 31) - 1 {
            return 0;
        }
        res as i32
    }
}

```
