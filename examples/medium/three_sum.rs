#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<i32>,
    expected: Vec<Vec<i32>>,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let nums = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            // assert_eq!(expected.clone(), two_sum(nums.clone(), *target))

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            assert_eq_unordered!(expected.clone(), three_sum(nums.clone()));
          }
        )*
    };
}

lazy_static! {
  // Edit this with your tests cases
  static ref TEST_DATA: Mutex<HashMap<&'static str, TestData>> = {
    let mut m = HashMap::new();
    m.insert(
      "example_1",
      TestData {
        input: vec![-1,0,1,2,-1,-4],
        expected: vec![vec![-1,-1,2], vec![-1,0,1]]
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![0,1,1],
        expected: vec![]
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: vec![0,0,0],
        expected: vec![vec![0, 0, 0]]
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted = nums.clone();
    sorted.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let n = sorted.len();

    for i in 0..n-2 {
      if i > 0 && sorted[i] == sorted[i - 1] {
        continue;
      }
      let mut l = i + 1;
      let mut r = n - 1;
      while l < r {
        let sum = sorted[i] + sorted[l] + sorted[r];
        if sum == 0 {
          result.push(vec![sorted[i], sorted[l], sorted[r]]);
          while l < r && sorted[l] == sorted[l + 1] {
            l += 1;
          }
          while l < r && sorted[r] == sorted[r - 1] {
            r -= 1;
          }
          l += 1;
          r -= 1;
        } else if sum < 0 {
          l += 1;
        } else {
          r -= 1;
        }
      }
    }
    return result;
}

// Add your tests here (use the same id as the key for the TEST_DATA)
run_test! {
  example_1,
  example_2,
  example_3,
}

fn main() {
    // Here we can run the code with log statements shown for debugging
    let test_data = &*TEST_DATA.lock().unwrap();
    for (name, test) in test_data.into_iter() {
        let nums = &test.input;
        let expected = &test.expected;
        let result = three_sum(nums.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
