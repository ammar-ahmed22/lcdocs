---
sidebar_position: 11
---

# Car Fleet
There are `n` cars at given miles away from the starting mile 0, traveling to reach the mile `target`.

You are given two integer array `position` and `speed`, both of length `n`, where `position[i]` is the starting mile of the `ith` car and `speed[i]` is the speed of the `ith` car in miles per hour.

A car cannot pass another car, but it can catch up and then travel next to it at the speed of the slower car.

A **car fleet** is a car or cars driving next to each other. The speed of the car fleet is the minimum speed of any car in the fleet.

If a car catches up to a car fleet at the mile `target`, it will still be considered as part of the car fleet.

Return the number of car fleets that will arrive at the destination.

## Examples
### Example 1
> Input: `target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]` 
> Output: `3` 
>
> Explanation: 
>  - The cars starting at 10 (speed 2) and 8 (speed 4) become a fleet, meeting each other at 12. The fleet forms at target.
>  - The car starting at 0 (speed 1) does not catch up to any other car, so it is a fleet by itself.
>  - The cars starting at 5 (speed 1) and 3 (speed 3) become a fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.

### Example 2
> Input: `target = 10, position = [3], speed = [3]` 
> Output: `1` 
>
> Explanation: 
>  - There is only one car, hence there is only one fleet.

### Example 3
> Input: `target = 100, position = [0, 2, 4], speed = [4, 2, 1]` 
> Output: `1` 
>
> Explanation: 
>  - The cars starting at 0 (speed 4) and 2 (speed 2) become a fleet, meeting each other at 4. The car starting at 4 (speed 1) travels to 5.
>  - Then, the fleet at 4 (speed 2) and the car at position 5 (speed 1) become one fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches `target`.

## Notes
### Conceptual Idea
- To start off, we'll sort the input arrays by position in DESCENDING order into a tuple array
  + This is because the cars closer to the target need to be handled first
- Create a stack
- Iterate over the sorted, descending array and calculate the time to target (`(target - pos) / speed`)
- If the stack is empty OR if the current time is greater than the top of the stack, push to the stack
  + If the current time is greater than the last time added to the stack, this is a new fleet (slow car has big time)
  + The first car will always create a fleet
- Return the length of the stack at the end

### Complexity
#### Time
- Sorting the arrays -> O(nlogn)

#### Space
- Creating a new sorted, coupled array -> O(n)

## Solution
### Rust
```rust
fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
  let mut coupled: Vec<(i32, i32)> = Vec::new();
  for i in 0..position.len() {
    coupled.push((position[i], speed[i]));
  }
  coupled.sort_by(|a, b| b.0.cmp(&a.0));
  let mut stack: Vec<f64> = Vec::new();
  for (pos, speed) in coupled.iter() {
    let time_to_target: f64 = (target - pos) as f64 / *speed as f64;
    if stack.is_empty() || time_to_target > stack[stack.len() - 1] {
      stack.push(time_to_target);
    }
  }
  return stack.len() as i32; 
}
```