#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::{ HashMap, HashSet };
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<i32>,
    expected: i32,
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
            assert_eq!(expected.clone(), longest_consecutive(nums.clone()))

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            // assert_eq_unordered!(expected.clone(), two_sum(nums.clone(), *target));
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
        input: vec![100,4,200,1,3,2],
        expected: 4
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![0,3,7,2,5,8,4,6,0,1],
        expected: 9
      }
    );

    // m.insert(
    //   "example_3",
    //   TestData {
    //     input: (vec![3,3], 6),
    //     expected: vec![0, 1]
    //   }
    // );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn longest_consecutive(nums: Vec<i32>) -> i32 {
  let set: HashSet<i32> = nums.clone().into_iter().collect();
  let mut max_seq = 0;
  for num in nums.iter() {
    let one_less = num - 1;
    if !set.contains(&one_less) {
      let mut temp = *num;
      let mut curr_seq = 0;
      while set.contains(&temp) {
        temp += 1;
        curr_seq += 1;
      }
      max_seq = std::cmp::max(max_seq, curr_seq);
    }
  }
  
  return max_seq;        
}

// Add your tests here (use the same id as the key for the TEST_DATA)
run_test! {
  example_1,
  example_2,
}

fn main() {
    // Here we can run the code with log statements shown for debugging
    let test_data = &*TEST_DATA.lock().unwrap();
    for (name, test) in test_data.into_iter() {
        let nums = &test.input;
        let expected = &test.expected;
        let result = longest_consecutive(nums.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
