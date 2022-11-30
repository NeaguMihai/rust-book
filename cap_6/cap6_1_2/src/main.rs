// enum Option<T> {
//     Mby(T)
// }

fn main() {
  // let test1 = Option::Mby(5);
  let test = 5;
  match test {
    5 => first_function(),
    _ => first_function_1(),
  }
}

fn first_function() {
  println!("Hello, world!");
}

fn first_function_1() {
  println!("Hello, world1!");
}