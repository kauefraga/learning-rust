const PI:f32 = 3.1415;
static mut GLOBAL:u8 = 1;

fn sum(x:i32, y:i32) -> i32 {
  x + y // dont use ; to "return"
}

fn anonym_scope() {
  let age:i8 = 18;

  {
    let age:i8 = 30;

    println!("Inside, age = {}", age);
  }

  println!("Outside, age = {}", age);
}

fn conditions(age:i8) {
  if age > 18 {
    println!("You can go on the party");
  } else {
    println!("You can't go on the party");
  }

  let conditional = if age > 18 { "older" } else { "younger" };

  println!("You is {} than 18", conditional);

  // Match statement
  let language = "C";
  let purpose = match language {
    "PHP" => "Web",
    "Kotlin" => "Mobile",
    "Python" => "Data Science",
    _ => "Unknown language" // "default" case
  };

  println!("Purpose of {} is {}", language, purpose);
}

fn loops() {
  let mut count: i8 = 0;
  let multiplicator: i8 = 6;

  while count < 10 {
    count += 1;

    if count == 6 {
      continue; // Jump to the next loop (6x6 does not exists)
    }

    println!("{} x {} = {}", multiplicator, count, count * multiplicator);
  }

  // loop is infinite
  count = 0;
  loop {
    count += 1;
    println!("{} x {} = {}", multiplicator, count, count * multiplicator);

    if count == 10 {
      break; // that's the way to break out of loop
    }
  }

  count = 0;
  for i in 1..11 { // or `in 1..=10`
    count += 1;
    println!("{} x {} = {}", i, count, count * i);
  }
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

  anonym_scope();

  println!("Sum is {}", sum(1, 2));

  conditions(age);

  loops();
}
