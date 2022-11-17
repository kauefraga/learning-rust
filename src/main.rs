fn main() {
  println!("Hello, world!");

  let age:i8 = 14;

  println!("Your age is {}", age);

  let is_true:bool = false;

  println!("Is it true? {}", is_true);

  let mut letter:char = 'c';
  letter = 'f';

  println!("Your letter is {} and the size of char is {} bytes", letter, std::mem::size_of_val(&letter));
}
