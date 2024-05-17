use colored::*;


pub struct TestCase<I, O> {
  input: I,
  output: O 
}

#[allow(dead_code)]
impl <I, O> TestCase<I, O> 
where 
  I: Clone,
  O: PartialEq + std::fmt::Debug
{
  
  pub fn new(input: I, output: O) -> Self {
    Self {
      input,
      output
    }
  }
  
  pub fn run<F>(&self, func:F) -> bool 
  where 
    F: FnOnce(I) -> O,
  {
    func(self.input.clone()) == self.output
  }

  pub fn run_and_log<F>(&self, func: F, id: String) 
  where 
    F: FnOnce(I) -> O,
  {
    let func_result = func(self.input.clone());
    let result = func_result == self.output;
    let icon = if result { "\u{2713}" } else { "\u{2717}" };
    let log = format!("Test Case {} {}", id, icon);
    
    println!("{}", if result { log.green() } else { log.red() });
    if !result {
      let expected = "Expected".truecolor(119, 119, 119);
      let expected_value = format!("{:>2?}", self.output).green();
      let received = "Received".truecolor(119, 119, 119);
      let received_value = format!("{:>2?}", func_result).red();
      println!("{}:\n{}\n{}:\n{}\n", expected, expected_value, received, received_value);
    }
  }
}

pub struct UnorderedVec<T>(pub Vec<T>);

impl <T: PartialEq> PartialEq for UnorderedVec<T> 
  where T: Clone + Ord
{
  fn eq(&self, other: &Self) -> bool {
      if self.0.len() != other.0.len() {
        return false;
      }
      let mut sorted_self = self.0.clone();
      sorted_self.sort();
      let mut sorted_other = other.0.clone();
      sorted_other.sort();
      for (a, b) in sorted_self.iter().zip(sorted_other.iter()) {
        if a != b {
          return false;
        }
      }
      return true
  }
}

impl<T: std::fmt::Debug> std::fmt::Debug for UnorderedVec<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_tuple("MyVec")
          .field(&self.0)
          .finish()
  }
}