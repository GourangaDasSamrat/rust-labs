fn main() {
    // Refutable pattern: Some(x) does not match when the value is None.
    let maybe_number: Option<i32> = Some(42);
    if let Some(value) = maybe_number {
        println!("Matched with if let: {value}");
    }

    // Irrefutable pattern: x matches any value, so it can be used in let.
    let x = 5;
    println!("Irrefutable binding: {x}");

    // let ... else is the safe way to handle a refutable pattern.
    let some_option_value: Option<i32> = None;
    let Some(number) = some_option_value else {
        println!("Handled the missing value with else.");
        return;
    };
    println!("Matched number: {number}");

    // match arms are refutable by default; the final arm can be catch-all.
    let name = Some("Ada");
    match name {
        Some(n) => println!("Hello, {n}!"),
        None => println!("No name provided."),
    }

    // Invalid examples from the chapter are intentionally commented out:
    // let Some(y) = some_option_value;
    // let x = 5 else { return; };
}
