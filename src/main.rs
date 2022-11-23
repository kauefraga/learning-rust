use learning_rust::{fs, os};

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

fn ownership() {
  // Static string = &static str
  // let static_string = "static Kaue";

  // Dynamic string
  let mut string = String::from("Kaue"); // Allocate pointer to string in stack and initialize the string in heap

  stole(&mut string);

  println!("{}", string);
}

fn stole(string: &mut String) {
  string.push_str(" Fraga");
  println!("{}", string);
}

fn pattern_matching() {
  for x in 1..=20 {
    println!("{}: {}", x, match x {
      1 => "Pouco",
      2 | 3 => "Um pouquinho",
      4..=10 => "Um bocado",
      _ if x % 2 == 0 => "Uma boa quantidade",
      _ => "Muito",
    })
  }
}

fn error() {
  match result() {
    Ok(s) => println!("String de sucesso = {}", s),
    Err(number) => println!("Código de erro = {}", number),
  }
  // let v = vec![1, 2, 3];
}

fn result() -> Result<String, u8> {
  Ok(String::from("Tudo deu certo"))
}

fn file_system() {
  let path = "./test.json";

  match fs::is_file(path) {
    true => {
      let file = fs::read_file(path)
        .expect("Should be able to read the file");

      println!("the file text is {}", file);
    },
    _ => println!("Nothing for you!")
  }
}

fn os_monitor() {
  os::get_ram();
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

  ownership();

  pattern_matching();

  error();

  file_system();

  os_monitor();
}
