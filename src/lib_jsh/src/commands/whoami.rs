use crate::commands::api::Command;

pub fn build_command() -> Command {
    let callback = move |input: String| {
        println!("Hello World")
    };
    Command::new(String::from("whoami"), Box::new(callback))
}
