// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

pub mod macros {
    #[macro_export]
    macro_rules! my_macro {
        ($val:expr) => (
            "Hello".to_string() + &String::from($val)
        )
    }
}


fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
