extern crate num;

use std::io;
use num::bigint::BigInt;
use num::FromPrimitive;

struct FibonacciType {
  current: BigInt,
  next: BigInt,
}

impl Iterator for FibonacciType {
  type Item = BigInt;

  fn next(&mut self) -> Option<Self::Item> {
    let new_next = &self.current + &self.next;

    self.current = self.next.clone();
    self.next = new_next.clone();

    Some(self.current.clone())
  }
}

fn fibonacci() -> FibonacciType {
  FibonacciType {
    current: FromPrimitive::from_u32(0).unwrap(),
    next: FromPrimitive::from_u32(1).unwrap(),
  }
}

fn negative_fibonacci() -> FibonacciType {
  FibonacciType {
    current: FromPrimitive::from_i32(0).unwrap(),
    next: FromPrimitive::from_i32(-1).unwrap(),
  }
}

fn main() {
  println!("Please input a number for fibonacci iterator:");

  let mut fib_input = String::new();
  let mut fib_result: BigInt = FromPrimitive::from_u32(0).unwrap();

  io::stdin().read_line(&mut fib_input)
    .expect("Failed to read line");

    // Match a signed integer. If positive, call to fibonacci() iterator. If negative, call to negative_fibonacci() iterator
    match fib_input.trim().parse::<i32>() {
    Ok(num) if num >= 0 => {
      // Threat the infinite iterator as finite, with take(). Runs the iterator with "n" terms.
      for it_result in fibonacci().take(num as usize) {
        fib_result = it_result;
      }
    },
    Ok(num) => {
      // Threat the infinite iterator as finite, with take(). Runs the iterator with "n" terms.
      for it_result in negative_fibonacci().take(num.abs() as usize) {
        fib_result = it_result;
      }
    },
    Err(_) => println!("Error: not type number."),
  };

  println!("Result: {}", fib_result);
}
