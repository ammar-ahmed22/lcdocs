#[allow(unused_imports)]
use assert_unordered::assert_eq_unordered;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Edit this to house the correct inputs and expected values
#[derive(Debug)]
struct TestData {
    input: Vec<String>,
    expected: i32,
}

macro_rules! run_test {
    ($($name:ident, )*) => {
        $(
          #[test]
          fn $name() {
            let test_data = &*TEST_DATA.lock().unwrap();
            let test = test_data.get(stringify!($name)).unwrap();
            let tokens = &test.input; // Edit this to extract your inputs (they are references "&" so use accordingly)
            let expected = &test.expected;

            // Use one of the below assertions to test the function
            // Use this if the order of the return value matters
            // e.g. [0, 1] != [1, 0]
            assert_eq!(expected.clone(), eval_rpn(tokens.clone()))

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
        input: vec!["2","1","+","3","*"].iter().map(|s| s.to_string()).collect(),
        expected: 9
      }
    );

    m.insert(
      "example_2",
      TestData {
        input: ["4","13","5","/","+"].iter().map(|s| s.to_string()).collect(),
        expected: 6
      }
    );

    m.insert(
      "example_3",
      TestData {
        input: ["10","6","9","3","+","-11","*","/","*","17","+","5","+"].iter().map(|s| s.to_string()).collect(),
        expected: 22
      }
    );

    Mutex::new(m)
  };
}


fn eval(a: i32, b: i32, op: &String) -> i32 {
  match op.as_str() {
    "+" => {
      return a + b;
    },
    "-" => {
      return a - b;
    },
    "/" => {
      return a / b;
    },
    "*" => {
      return a * b;
    },
    _ => {
      return 0;
    }
  }
}

// Edit this function for the problem
fn eval_rpn(tokens: Vec<String>) -> i32 {
  let mut stack: Vec<i32> = Vec::new();
  for char in tokens.iter() {
    match char.parse::<i32>() {
      Ok(number) => {
        stack.push(number);
      },
      Err(_) => {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(eval(a, b, char));
      } 
    }
  }
  return stack[stack.len() - 1];
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
        let tokens = &test.input;
        let expected = &test.expected;
        let result = eval_rpn(tokens.clone());
        println!("Running '{}'", name);
        println!("expected: {:?}, received: {:?}", expected, result);
    }
}
