#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<i32>,
    expected: Vec<i32>,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let temperatures = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), daily_temperatures(temperatures.clone()));

            // Using this because the order of the return value does not matter!
            // e.g. [0, 1] == [1, 0]
            // cloning vectors and dereferencing integers
            // assert_eq_unordered!(expected.clone(), daily_temperatures(temperatures.clone()));
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
        input: vec![73,74,75,71,69,72,76,73],
        expected: vec![1,1,4,2,1,1,0,0]
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: vec![30,40,50,60],
        expected: vec![1,1,1,0]
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: vec![30,60,90],
        expected: vec![1,1,0]
      }
    );

    Mutex::new(m)
  };
}

// Edit this function for the problem
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut stack: Vec<(i32, usize)> = Vec::new();
    let mut result: Vec<i32> = vec![0; n];
    for (i, curr) in temperatures.iter().enumerate() {
      while stack.len() > 0 && curr > &stack[stack.len() - 1].0 {
        if let Some((_, idx)) = stack.pop() {
          result[idx] = (i - idx) as i32;
        }
      }
      stack.push((*curr, i)); 
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
        let temperatures = &test.input;
        let expected = &test.expected;
        let result = daily_temperatures(temperatures.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
