mod test_case;
use test_case::{TestCase, UnorderedVec};
use anyhow::Result;

struct Solution {}

impl Solution {
  pub fn max_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    for value in nums.iter() {
      if value > &max {
        max = *value;
      }
    }

    return max;
  }

  /// Returns the same array it was passed
  pub fn ditto(nums: Vec<i32>) -> UnorderedVec<i32> {
    UnorderedVec(nums)
  }

}

fn main() -> Result<()> {
  let test_cases: Vec<TestCase<Vec<i32>, i32>> = vec![
    TestCase::new(vec![1, 2, 3, 8], 8),
    TestCase::new(vec![-1, -100, -2, 4], 4),
    TestCase::new(vec![-1, -100, -1000], -1)
  ];

  println!("Running test for 'max_array'");
  for (i, test_case) in test_cases.iter().enumerate() {
    test_case.run_and_log(Solution::max_array, (i + 1).to_string());
  }

  // Testing when the output array's order does not matter
  let ditto_test: Vec<TestCase<Vec<i32>, UnorderedVec<i32>>> = vec![
    TestCase::new(vec![1, 2], UnorderedVec(vec![1, 2])),
    TestCase::new(vec![2, 1], UnorderedVec(vec![2, 1])),
    TestCase::new(vec![1, 2], UnorderedVec(vec![2, 1])) // Here we see the order is flipped but the equality still holds.
  ];
  println!("\nRunning test for 'ditto'");
  for (i, test_case) in ditto_test.iter().enumerate() {
    test_case.run_and_log(|v| Solution::ditto(v), (i + 1).to_string());
  }
  
  Ok(())
}