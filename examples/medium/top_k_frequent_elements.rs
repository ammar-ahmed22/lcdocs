#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: (Vec<i32>, i32),
    expected: Vec<i32>,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let (nums, k) = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            // assert_eq!(expected.clone(), two_sum(nums.clone(), *target))

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            assert_eq_unordered!(expected.clone(), top_k_frequent(nums.clone(), *k));
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
        input: (vec![1,1,1,2,2,3], 2),
        expected: vec![1, 2]
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: (vec![1], 1),
        expected: vec![1]
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: (vec![1, 2], 2),
        expected: vec![1, 2]
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
  let n = nums.len();
  // Count frequency
  let mut freq: HashMap<i32, usize> = HashMap::new();
  for num in nums.iter() {
    let updated = match freq.get(num) {
      Some(frequency) => {
        *frequency + 1
      },
      None => {
        1
      }
    };
    freq.insert(*num, updated);
  }
  
  // Update the count array
  let mut count: Vec<Vec<i32>> = vec![vec![]; n + 1];
  for (key, val) in freq.iter() {
    count[*val].push(*key);
  }

  // Return the k most frequent elements
  let mut result: Vec<i32> = vec![];
  let mut i: usize = n;
  while result.len() < (k as usize) {
    for val in count[i].iter() {
      result.push(*val);
    }
    i -= 1;
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
        let (nums, k) = &test.input;
        let expected = &test.expected;
        let result = top_k_frequent(nums.clone(), *k);
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
