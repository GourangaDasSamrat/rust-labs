// This file shows a declarative macro, which is the most common form of macro in Rust.
// A macro can take a variable number of arguments and expand into more Rust code
// before the compiler checks it. This is different from a function, which must have
// a fixed signature and is called at runtime.
//
// The pattern below is similar to the vec! macro from the Rust book:
// it matches any number of expressions and generates code to push each value into
// a Vec, then returns the vector.
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Another small example: a macro with multiple patterns.
// This one creates different output depending on the number of arguments.
#[macro_export]
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    ($first:expr, $last:expr) => {
        println!("Hello, {} {}!", $first, $last);
    };
}

// Procedural macros are covered later in the book. They are different because they
// are written as functions that receive and return Rust token streams, and the
// definitions often live in their own crate with `proc-macro = true`.
// We keep this example simple and focus on declarative macros here.
