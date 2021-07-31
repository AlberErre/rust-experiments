use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
  value: T,
  next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
  fn set_next(&mut self, next_node: Node<T>) {
    self.next = Some(Box::new(next_node));
  }
}

fn main() {
  println!("Hello, Linked list!");

  let mut head = Node {
    value: 1,
    next: None,
  };

  head.set_next(Node {
    value: 2,
    next: None,
  });

  println!("The linked list starting from head is: {:?}", head);

  println!("head node value: {:?}", head.value);
  println!("next node value: {:?}", head.next.unwrap().value);
}
