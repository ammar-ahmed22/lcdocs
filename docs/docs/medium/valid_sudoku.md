---
sidebar_position: 4
---

# Valid Sudoku
Determine if a `9 x 9` Sudoku board is valid. Only the filled cells need to be validated **according to the following rules**:

1. Each row must contain the digits `1-9` without repetition.
2. Each column must contain the digits `1-9` without repetition.
3. Each of the nine 3 x 3 sub-boxes of the grid must contain the digits `1-9` without repetition.

**Note**:

- A Sudoku board (partially filled) could be valid but is not necessarily solvable.
- Only the filled cells need to be validated according to the mentioned rules.

## Examples
### Example 1
> `Input: board =` <br /> 
` [["5","3",".",".","7",".",".",".","."]` <br /> 
` ,["6",".",".","1","9","5",".",".","."]` <br /> 
` ,[".","9","8",".",".",".",".","6","."]` <br /> 
` ,["8",".",".",".","6",".",".",".","3"]` <br /> 
` ,["4",".",".","8",".","3",".",".","1"]` <br /> 
` ,["7",".",".",".","2",".",".",".","6"]` <br /> 
` ,[".","6",".",".",".",".","2","8","."]` <br /> 
` ,[".",".",".","4","1","9",".",".","5"]` <br /> 
` ,[".",".",".",".","8",".",".","7","9"]]` <br />
> `Output: true`

### Example 2
> `Input: board =` <br /> 
` [["8","3",".",".","7",".",".",".","."]` <br /> 
` ,["6",".",".","1","9","5",".",".","."]` <br /> 
` ,[".","9","8",".",".",".",".","6","."]` <br /> 
` ,["8",".",".",".","6",".",".",".","3"]` <br /> 
` ,["4",".",".","8",".","3",".",".","1"]` <br /> 
` ,["7",".",".",".","2",".",".",".","6"]` <br /> 
` ,[".","6",".",".",".",".","2","8","."]` <br /> 
` ,[".",".",".","4","1","9",".",".","5"]` <br /> 
` ,[".",".",".",".","8",".",".","7","9"]]` <br />
> `Output: false` <br />
> **Explanation**: Same as Example 1, except with 5 in top-left corner changed to 8, resulting in the 3x3 box having two 8's.

## Notes
### Conceptual Idea
- Essentially, we need to find duplicates in any row, column or 3x3 sub-box. 
- If a duplicate is found, then it is an invalid Sudoku
- We can iterate over each row, column and 3x3 sub-box with a hashset, checking for duplicates
- We can have 3 loops, one for the rows, one for the columns and one for the 3x3 sub-boxes. 
- For the rows:
  - Iterate over the vector, each value is a row -> iterate checking for duplicates
  - e.g: 
  - ```rust
      for row in board.iter() { 
        set.clear()
        for c in row.iter() { 
          // check for duplicates 
        } 
      }
    ```
- For the cols:
  - For loop from 0 - 9, -> i
    - Clear the hashset
    - For loop from 0 - 9 -> j
      - `board[j][i]` -> check for duplicates
- For the sub-boxes:
  - For loop from 0 - 3 -> box_row
    - For loop from 0 - 3 -> box_col
      - Clear the hashset
      - For loop from 0 - 3 -> row
        - For loop from 0 - 3 -> col
          - `board[box_row * 3 + row][box_col * 3 + col]` -> check for duplicates
- We can use a single hashset to check for all the duplicates by clearing it on each iteration for row, col and sub-box

### Complexity
#### Time 
- Let's say the sudoku matrix length is `n`, we do multiple consecutive nested loops -> O(n^2)
#### Space
- The only extra space we create is the hashset which will have constant length -> O(1)

## Solution
### Rust
```rust
fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  let mut set: HashSet<char> = HashSet::new();
  
  // Checking rows
  for row in board.iter() {
    set.clear();
    for char in row.iter() {
      if *char != '.' {
        if set.contains(char) {
          return false;
        } else {
          set.insert(*char);
        } 
      }
    }
  }

  // Checking cols
  for i in 0..9 {
    set.clear();
    for j in 0..9 {
      let char = board[j][i];
      if char != '.' {
        if set.contains(&char) {
          return false;
        } else {
          set.insert(char);
        }
      }
    }
  }

  // Checking sub-boxes
  for box_row in 0..3 {
    for box_col in 0..3 {
      set.clear();
      for row in 0..3 {
        for col in 0..3 {
          let char = board[box_row * 3 + row][box_col * 3 + col];
          if char != '.' {
            if set.contains(&char) {
              return false;
            } else {
              set.insert(char);
            }
          }
        }
      }
    }
  }

  return true;
}
```