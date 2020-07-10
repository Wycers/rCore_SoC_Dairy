# rCore_SoC_Dairy

# 2020-7-8

## 回顾

- 用 Rust 刷了 LeetCode #2 #3 #7

## Lab

……

## 刷题

### LeetCode 6. ZigZag Conversion

#### cpp version

```cpp
class Solution {
public:
    string convert(string str, int n) {
        if (n == 1)
            return str;
        string res = "";
        int step = (n - 1) << 1;
        int len = str.length();
        for (int i = 0; i < n; ++i)
        {
            if (i == 0 || i == n - 1)
                for (int j = i; j < len; j += step)
                    res += str[j];
            else
                for (int j = i; j < len; j += step)
                {
                    res += str[j];
                    if (j + step - (i << 1) < len)
                        res += str[j + step - (i << 1)];
                }
        }
        return res;
    }
};

```

#### rust version

```rust

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

```
