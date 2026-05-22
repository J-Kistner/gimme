use std::io;

pub fn user_input() -> String {
   let mut input = String::new();
   io::stdin()
      .read_line(&mut input)
      .expect("Failed to get user input");
   if input.len() > 0 { input } else { user_input() }
}
