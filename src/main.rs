use std::ffi::OsString;
use users::get_current_username;

fn main() {
    let username;
    match get_current_username() {
        Some(uname)=>username=uname,
        None=>username=OsString::from("default user"),
    }
    println!("Hello {:#?}! This is the Monkey programming language implemented in Rust!\n", username);
}
