#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: (i32, Vec<i32>, Vec<i32>),
    expected: i32,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let (target, pos, speed) = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), car_fleet(*target, pos.clone(), speed.clone()))

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
        input: (12, vec![10,8,0,5,3], vec![2,4,1,1,3]),
        expected: 3
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: (10, vec![3], vec![3]),
        expected: 1
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: (100, vec![0,2,4], vec![4,2,1]),
        expected: 1
      }
    );

    m.insert(
      "example_4",
      TestData {
        input: (10, vec![6, 8], vec![3, 2]),
        expected: 2
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
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

// Add your tests here (use the same id as the key for the TEST_DATA)
run_test! {
  example_1,
  example_2,
  example_3,
  example_4,
}

fn main() {
    // Here we can run the code with log statements shown for debugging
    let test_data = &*TEST_DATA.lock().unwrap();
    for (name, test) in test_data.into_iter() {
        let (target, pos, speed) = &test.input;
        let expected = &test.expected;
        let result = car_fleet(*target, pos.clone(), speed.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
