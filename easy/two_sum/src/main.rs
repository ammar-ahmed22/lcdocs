mod test_case;
use test_case::{TestCase, UnorderedVec};
use anyhow::Result;

struct Solution {}

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> UnorderedVec<i32> {
    let mut hash: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for (i, n) in nums.iter().enumerate() {
      hash.insert(*n, i as i32);
    }

    for (i, n) in nums.iter().enumerate() {
      let t = target - n;
      if hash.contains_key(&t) {
        let idx = match hash.get(&t) {
          Some(i) => *i,
          None => -1
        };
        if idx != -1 && idx != (i as i32) {
          return UnorderedVec(vec![idx, i as i32]);
        }
      }
    }

    return UnorderedVec(vec![0, 0])
  }
}

fn main() -> Result<()> {
  let test_cases: Vec<TestCase<(Vec<i32>, i32), UnorderedVec<i32>>> = vec![
    TestCase::new((vec![2,7,11,15], 9), UnorderedVec(vec![0, 1])),
    TestCase::new((vec![3,2,4], 6), UnorderedVec(vec![1, 2])),
    TestCase::new((vec![3, 3], 6), UnorderedVec(vec![0, 1]))
  ];

  println!("Running test for 'two_sum'");
  for (i, test_case) in test_cases.iter().enumerate() {
    test_case.run_and_log(|(a, b)| Solution::two_sum(a, b), (i + 1).to_string());
  }
  
  Ok(())
}