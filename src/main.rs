const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn sum(x:i32, y:i32) -> i32 {
  x + y // dont use ; and "return"
}

fn other_function() {
  let age:i8 = 18;

  {
    let age:i8 = 30;

    println!("Inside, age = {}", age);
  }

  println!("Outside, age = {}", age);
}

fn main() {
  println!("PI = {}", PI);

  unsafe {
    println!("variavel global = {}", GLOBAL);
  }

  println!("Hello, world!");

  let age:i8 = 14;

  println!("Your age is {}", age);

  let is_true:bool = false;

  println!("Is it true? {}", is_true);

  let mut letter:char = 'c';
  letter = 'f';

  println!("Your letter is {} and the size of char is {} bytes", letter, std::mem::size_of_val(&letter));

  other_function();

  println!("Sum is {}", sum(1, 2));
}
