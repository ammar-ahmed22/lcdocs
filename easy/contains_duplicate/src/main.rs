mod test_case;
use test_case::TestCase;

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
}



fn main() {
  let test_cases: Vec<TestCase<Vec<i32>, i32>> = vec![
    TestCase::new(vec![1, 2, 3, 8], 8),
    TestCase::new(vec![-1, -100, -2, 4], 4),
    TestCase::new(vec![-1, -100, -1000], -10)
  ];

  for (i, test_case) in test_cases.iter().enumerate() {
    test_case.run_and_log(|v| Solution::max_array(v), (i + 1).to_string());
  }
}