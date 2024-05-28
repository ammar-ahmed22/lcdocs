---
sidebar_position: 8
---

# Container With Most Water
You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the `ith` line are `(i, 0)` and `(i, height[i])`.

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the *maximum amount of water a container can store*.

**Notice** that you may not slant the container.



## Examples
### Example 1
![Container With Most Water Example](./assets/container_most_water.jpeg)

> `Input: height = [1,8,6,2,5,4,8,3,7]` <br />
> `Output: 49` <br />
> `Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.`

### Example 2
> `Input: height = [1, 1]` <br />
> `Output: 1`

## Notes
### Conceptual Idea
- Keep track of the max area with a variable
- Initialize two pointers, one at the start and one at the end
  + On each iteration, calculate the area and keep track of the max
    - Area = width (pointer separation) * min_height (minimum of the heights of 2 pointer values)
  + If the left value is smaller than the right, increment the left
  + If the right value is smaller than left, decrement the right
  + If they are the same, increment left and decrement right
    + This allows for the max area to be found because we keep the largest height for as long as possible

### Complexity
#### Time
- Only iterate once -> O(n)

#### Space
- No new space -> O(1)

## Solution
### Rust
```rust
fn calc_area(l_ptr: usize, l_val: i32, r_ptr: usize, r_val: i32) -> i32 {
  let height = std::cmp::min(l_val, r_val);
  let width = (r_ptr - l_ptr) as i32;
  return width * height;
}

// Problem question
fn max_area(height: Vec<i32>) -> i32 {
  let mut max = 0;
  let mut l = 0 as usize;
  let mut r = height.len() - 1;

  while l < r {
    let l_val = height[l];
    let r_val = height[r];
    max = std::cmp::max(max, calc_area(l, l_val, r, r_val));
    if l_val == r_val {
      r -= 1;
      l += 1;
    } else if l_val < r_val {
      l += 1;
    } else {
      r -= 1;
    }
   }
  return max;
}
```