---
sidebar_position: 5
---

# Longest Consecutive Sequence
Given an unsorted array of integers `nums`, return the *length of the longest consecutive elements sequence*.

You must write an algorithm that runs in `O(n)` time.

## Examples
### Example 1
> `Input: nums = [100,4,200,1,3,2]` <br />
> `Output: 4` <br />
> `Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.`

### Example 2
> `Input: nums = [0,3,7,2,5,8,4,6,0,1]` <br />
> `Output: 9`

## Notes
### Conceptual Idea
- The general idea we can use is that on each number, we want to look for the number right after it in the array, if it doesn't exist, that sequence is over right there
- Create a hashset and put all the numbers in the array in there -> O(n)
- Iterate over the numbers
  + Check if the set has the number one less than the current, if it doesn't that means it is the smallest value in the sequence
    - Do a while loop which iterates while the set has the value one more than the current value, update the value on each iteration and keep looking
    - Keep track of the current sequence length and the max sequence length
- Return the max sequence length

### Complexity
#### Time
- Iterate once to create hashset -> O(n)
- In the main iteration, you may think it's O(n^2) but its not
  + Since we only do the while loop when it's the smallest number in the sequence, that will only happen once for the largest sequence, retaining O(n)

#### Space
- O(n) -> hashset

## Solution
### Rust
```rust
fn main() {
  println!("Hello world!");
}
```