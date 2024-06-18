---
sidebar_position: 12
---

# Search 2D Matrix
You are given an `m x n` integer matrix `matrix` with the following two properties:

- Each row is sorted in non-decreasing order.
- The first integer of each row is greater than the last integer of the previous row.

Given an integer `target`, return `true` if `target` is in `matrix` or `false` otherwise.

You must write a solution in `O(log(m * n))` time complexity.



## Examples
### Example 1
> Input: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3`  
> Output: `true`

### Example 2
> Input: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13`  
> Output: `false` 

## Notes
### Conceptual Idea
- Essentially need to to do binary search twice, once to find the row and once to find the value
- Initialize top and bottom pointers for the row binary search
- Iterate while top is less than or equal to bottom
  + Calculate the midpoint
  + Check if the row on the midpoint contains the value (the first value is less than or equal to the target and the last value is greater than or equal to the target)
    - If the row contains the value, do the edge case check if the ends of the row are the target and return the true
    - Do another binary search on the row to find the target and return false if it doesn't
  + Check if the first value on the midpoint row is less than the target
    - Update the top pointer to mid + 1
  + Else
    - Update the bottom pointer to mid - 1

### Complexity
#### Time
- We do a nested binary search first on the m-size vec and then on the n-size vec
  + O(log(m * n))

#### Space
- O(1)

## Solution
### Rust
```rust
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
   let m = matrix.len();
   let n = matrix[0].len();
   let mut top = 0 as usize;
   let mut bot = m - 1 as usize;

   while top < m && bot < m && top <= bot {
    let mid = (top + bot) / 2;
    if matrix[mid][0] <= target && matrix[mid][n - 1] >= target {
      if matrix[mid][0] == target || matrix[mid][n - 1] == target {
        return true;
      }
      let mut l = 0 as usize;
      let mut r = n - 1;
      while l < n && r < n && l <= r {
        let cen = (l + r) / 2;
        if matrix[mid][cen] == target {
          return true;
        } else if matrix[mid][cen] < target {
          l = cen + 1;
        } else {
          r = cen - 1;
        }
      }
      return false;
    } else if matrix[mid][0] < target {
      top = mid + 1;
    } else {
      bot = mid - 1;
    }
   }
   return false;
}
```