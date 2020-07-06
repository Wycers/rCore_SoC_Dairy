# rCore_SoC_Dairy

# 2020-7-6

## 回顾

- 完成了一个最小的 riscv os 镜像，并在 qemu 上运行。
- 完成了 RISC-V 中有关中断处理的部分。

## Lab

要开始内存管理了。

为了方便起见，使用现成的`buddy_system_allocator`

```yaml
# Cargo.toml

[dependencies]
---
buddy_system_allocator = "0.3.9"
```

## 刷题

### LeetCode 1. Two Sum

#### cpp version

```cpp
class Solution
{
public:
    map<int, int> mp;
    vector<int> twoSum(vector<int> &nums, int target)
    {
        mp.clear();
        vector<int> res;
        for (int i = 0; i < nums.size(); ++i)
        {
            int need = target - nums[i];
            map<int, int>::iterator iter = mp.find(need);
            if (iter != mp.end())
            {
                res.push_back(mp[iter->first]);
                res.push_back(i);
                break;
            }
            mp[nums[i]] = i;
        }
        return res;
    }
};
```

#### rust version

```rust

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        let mut res = vec![];
        for (i, v) in nums.iter().enumerate() {
            match map.get(&(target - v)) {
                None => {
                    map.insert(v, i);
                }
                Some(j) {
                    res = vec![*j as i32, i as i32];
                }
            }
        }
        res
    }
}

```

看着一个 os 从零到开始，真的真的好开心啊。
