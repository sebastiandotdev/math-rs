#[derive(Debug)]
pub struct Sum {}

impl Sum {
  pub fn new(nums: Vec<i32>) -> i32 {
    let mut total = 0;

    for num in nums.iter() {
      total += num
    }

    return total;
  }
}

#[test]
fn test_sum() {
  let result = Sum::new(vec![2, 2]);

  assert_eq!(result, 4);
}

#[test]
fn test_sum_error() {
  let result = Sum::new(vec![2, 2]);

  assert_ne!(result, 5);
}
