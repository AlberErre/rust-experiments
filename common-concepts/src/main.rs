fn main() {
    let vector = get_vector();
    println!("Hello, vector!, {:?}", vector);
}


fn get_vector() -> Vec<i32> {
  vec![1,2,4]
}