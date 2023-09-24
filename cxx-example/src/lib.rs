use cxx::CxxString;

pub mod tester;
use crate::tester::this_logs;

#[cxx::bridge]
mod ffi {
    #[namespace = "rust_part"]
    extern "Rust" {
        fn log_me(msg: &CxxString);
    }
}

fn log_me(msg: &CxxString) {
    println!("I'm a log! {}", msg);
    this_logs();
}