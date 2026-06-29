struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // This code runs automatically when the value goes out of scope.
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // Create two values that own heap data and implement Drop.
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");

    // `d` and `c` are dropped automatically at the end of `main`.
    // Rust drops values in reverse order of creation, so `d` is dropped first.
    drop(c);

    // We can force early cleanup with the `std::mem::drop` function.
    // This calls the destructor logic immediately for the given value.
    println!("CustomSmartPointer `c` dropped before the end of main");

    // `d` is still alive until the end of the function scope.
    println!("End of main approaching");
}
