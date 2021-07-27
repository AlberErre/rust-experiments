fn main() {
  println!("Hello, is prime!");

  println!("{} is prime?, {:?}", 2, is_prime(2));
  println!("{} is prime?, {:?}", 4, is_prime(4));
  println!("{} is prime?, {:?}", 32, is_prime(32));
  println!("{} is prime?, {:?}", 37, is_prime(37));
  println!("{} is prime?, {:?}", 11, is_prime(11));
  println!("{} is prime?, {:?}", 97, is_prime(97));
  println!("{} is prime?, {:?}", 3456, is_prime(3456));
}

fn is_prime(x: i32) -> bool {
  match x {
    x if x == 2 => true,
    x if x < 2 => false,
    x if x % 2 == 0 => false,
    _ => {
      let odd_numbers: Vec<i32> = (3..).step_by(2).take_while(|n| n < &x).collect();
      odd_numbers.iter().all(|n| x % n != 0)
    }
  }
}
