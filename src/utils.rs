use std::{
  fs::File,
  io::{self, Read},
};

pub fn get_code_from_file(file_path: &str) -> io::Result<String> {
  let mut lua_code = String::new();
  let mut file = File::create(file_path)?;
  file.read_to_string(&mut lua_code)?;
  Ok(lua_code)
}
