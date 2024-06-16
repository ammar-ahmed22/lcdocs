---
sidebar_position: 10
---

# Daily Temperatures
Given an array of integers `temperatures` represents the daily temperatures, return an array `answer` such that `answer[i]` is the number of days you have to wait after the `ith` day to get a warmer temperature. If there is no future day for which this is possible, keep `answer[i] == 0` instead.

## Examples
### Example 1
> `Input: temperatures = [73,74,75,71,69,72,76,73]` <br />
> `Output: [1,1,4,2,1,1,0,0]`

### Example 2
> `Input: temperatures = [30,40,50,60]` <br />
> `Output: [1,1,1,0]`

## Example 3
> `Input: temperatures = [30,60,90]` <br />
> `Output: [1,1,0]`

## Notes
### Conceptual Idea
- Create a stack
- Initialize the result with all zeroes
- Iterate over the numbers
- Do a while loop inside the iteration
  + while the stack is **not** empty **AND** the current number is greater than the stack top
  + Inside this loop, we'll update the result because we found the next day with a higher temperature
  + Pop from the stack and update the popped index with the difference of the indices
- Outside of the while loop, add the current number to the stack

### Complexity
#### Time
- We only iterate over the numbers once
- The while loop will only be, at max, the length of the largest index difference, so it's constant
- O(n)

#### Space
- O(n)

## Solution
### Rust
```rust
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut stack: Vec<(i32, usize)> = Vec::new();
    let mut result: Vec<i32> = vec![0; n];
    for (i, curr) in temperatures.iter().enumerate() {
      while stack.len() > 0 && curr > &stack[stack.len() - 1].0 {
        if let Some((_, idx)) = stack.pop() {
          result[idx] = (i - idx) as i32;
        }
      }
      stack.push((*curr, i)); 
    }

    return result;
}
```