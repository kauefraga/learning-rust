use std::{io, fs, path::Path};

pub fn is_file(path: &str) -> bool {
  let resolved_path = Path::new(path);

  resolved_path.is_file()
}

pub fn read_file(path: &str) -> io::Result<String> {
  let file = fs::read_to_string(path)
    .expect("Should have been able to read the file");

  Ok(file)
}
