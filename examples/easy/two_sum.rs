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
            let (nums, target) = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            // assert_eq!(expected.clone(), two_sum(nums.clone(), *target))

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            assert_eq_unordered!(expected.clone(), two_sum(nums.clone(), *target));
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
        input: (vec![2, 7, 11, 15], 9),
        expected: vec![0, 1]
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: (vec![3,2,4], 6),
        expected: vec![1, 2]
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: (vec![3,3], 6),
        expected: vec![0, 1]
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    println!("inside two_sum"); // This will not be shown in test mode, only when running
    let mut hash: HashMap<i32, usize> = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        let t = target - n;
        match hash.get(&t) {
            Some(val) => {
                if *val != i {
                    return vec![*val as i32, i as i32];
                }
            }
            None => {
                hash.insert(*n, i);
            }
        }
    }
    return vec![0, 0];
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
        let (nums, target) = &test.input;
        let expected = &test.expected;
        let result = two_sum(nums.clone(), *target);
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
