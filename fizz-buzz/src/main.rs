fn main() {
  println!("Hello, Fizz Buzz!");

  const NUMBER: i32 = 35;

  let fizz_buzz_result = fizz_buzz(NUMBER);

  println!("Fizz Buzz for number {} is: {:?}", NUMBER, fizz_buzz_result);
}

fn fizz_buzz(number: i32) -> Vec<String> {
  //NOTE: we start at 1 to avoid handling another case with 0
  let numbers = 1..=number;

  numbers
    .map(|n| match n {
      n if (n % 3 == 0 && n % 5 == 0) => String::from("FizzBuzz"),
      n if n % 3 == 0 => String::from("Fizz"),
      n if n % 5 == 0 => String::from("Buzz"),
      _ => n.to_string(),
    })
    .collect()
}
