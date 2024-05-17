mod test_case;
use test_case::{TestCase, UnorderedVec};
use anyhow::Result;

struct Solution {}

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    for num in nums.iter() {
      if hash.contains_key(&num) {
        return true;
      } else {
        hash.insert(*num, true);
      }
    }

    return false;
  }

}

fn main() -> Result<()> {
  let test_cases: Vec<TestCase<Vec<i32>, bool>> = vec![
    TestCase::new(vec![1,2,3,1], true),
    TestCase::new(vec![1,2,3,4], false),
    TestCase::new(vec![1,1,1,3,3,4,3,2,4,2], true)
  ];

  println!("Running test for 'contains_duplicate'");
  for (i, test_case) in test_cases.iter().enumerate() {
    test_case.run_and_log(Solution::contains_duplicate, (i + 1).to_string());
  }
  
  Ok(())
}