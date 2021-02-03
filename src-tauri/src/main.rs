#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

mod cmd;
fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      use cmd::Todo;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
              SpitTodos{name, date_time, grandparent, parent, id, callback, error} => tauri::execute_promise(
                  _webview, move || {
                      if date_time > 5 {
                          let response = Todo {
                              name: name,
                              date_time: date_time,
                              grandparent: grandparent,
                              parent: parent,
                              id: id,
                          };

                          Ok(response)
                              
                      } else {
                          Err(CommandError::new("datetime shoudl be greater than 5 bichass").into())
                      }
                  },callback, error, 
            ),
            MyCustomCommand { argument } => {

                println!("oh god oh fuck");
                println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
