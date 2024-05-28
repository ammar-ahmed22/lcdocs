---
sidebar_position: 7
---

# Three Sum
Given an integer array `nums`, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.

Notice that the solution set must not contain duplicate triplets.

## Examples
### Example 1
> `Input: nums = [-1,0,1,2,-1,-4]` <br />
> `Output: [[-1,-1,2],[-1,0,1]]`

### Example 2
> `Input: nums = [0,1,1]` <br />
> `Output: []`

### Example 3
> `Input: nums = [0,0,0]` <br />
> `Output: [[0, 0, 0]]`

## Notes
### Conceptual Idea
- We will use the same principle as [Two Sum Sorted](./two_sum_sorted.md), sort the input array
- Iterate up to second last item
- On each iteration, start off by checking if this number is a duplicate (we only want the last one) 
  + e.g. `[0, 0, 1, 2]`, we want to start at the 2nd `0`.
- Once we get to the last duplicate, initialize 2 pointers, one right in front of the current value (`left`) and the other at the end of the array (`right`)
  + Iterate while `left < right`,
    - Check if the 3 numbers sum to 0
      + Add to result array
      + Move the pointers until they are not duplicates
      + Move both pointers
    - If the sum is less than 0, move the left pointer
    - If the sum is greater than 0, move right pointer

### Complexity
#### Time
- Sorting input array -> O(n logn)
- Iterating over the numbers -> O(n)
- Results in O(n logn)

#### Space
- no new space created, O(1)

## Solution
### Rust
```rust
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted = nums.clone();
    sorted.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let n = sorted.len();

    for i in 0..n-2 {
      if i > 0 && sorted[i] == sorted[i - 1] {
        continue;
      }
      let mut l = i + 1;
      let mut r = n - 1;
      while l < r {
        let sum = sorted[i] + sorted[l] + sorted[r];
        if sum == 0 {
          result.push(vec![sorted[i], sorted[l], sorted[r]]);
          while l < r && sorted[l] == sorted[l + 1] {
            l += 1;
          }
          while l < r && sorted[r] == sorted[r - 1] {
            r -= 1;
          }
          l += 1;
          r -= 1;
        } else if sum < 0 {
          l += 1;
        } else {
          r -= 1;
        }
      }
    }
    return result;
}
```