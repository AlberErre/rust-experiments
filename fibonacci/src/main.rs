fn main() {
  println!("Hello, Fibonacci!");

  const FIBONACCI_SEQUENCE_INDEX: i32 = 8;

  let result = fibonacci(FIBONACCI_SEQUENCE_INDEX);

  println!(
    "Get fibonacci sequence number for index -> {}",
    FIBONACCI_SEQUENCE_INDEX
  );
  println!("result: {}", result);
}

fn fibonacci(n: i32) -> i32 {
  match n {
    n if (n == 1) => 0,
    n if (n == 2) => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}
