/*

    RUST CONTROL FLOW


    Control flow refers to the ability to:
    1. Run code conditionally based on whether a condition is true
    2. Run code repeatedly while a condition is true

    The main constructs are:
    - if expressions (conditional branching)
    - loop, while, for (repetition/looping)
*/

//
// 1. IF EXPRESSIONS
//

fn if_expressions_example() {
    println!("\n--- IF Expressions ---");

    // Basic if expression
    // Syntax: if condition { code_if_true } else { code_if_false }
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // IMPORTANT: The condition MUST be a bool (true/false)
    // Rust does NOT automatically convert integers to booleans like JavaScript or Ruby
    // This would cause a compile error:
    // if number { ... }  <- ERROR: expected bool, found integer

    // If you want to check if a number is not zero:
    if number != 0 {
        println!("number was something other than zero");
    }
}

//
// 2. ELSE IF - HANDLING MULTIPLE CONDITIONS
//

fn else_if_example() {
    println!("\n--- ELSE IF (Multiple Conditions) ---");

    let number = 6;

    // Using else if for multiple conditions
    // The program checks conditions in order and executes the FIRST matching block
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // This will execute
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // Won't reach here
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Note: Even though 6 is divisible by 2, that block doesn't execute
    // because the else if above it already matched

    // Tip: Using too many else if blocks clutters code
    // For many conditions, consider using 'match' (Chapter 6)
}

//
// 3. IF AS AN EXPRESSION (Assigning result to variable)
//

fn if_as_expression() {
    println!("\n--- IF as Expression ---");

    // In Rust, if is an EXPRESSION (not just a statement)
    // This means it returns a value!

    // Basic example
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // number is 5 because condition is true

    println!("The value of number is: {number}");

    // How it works:
    // - Blocks evaluate to the LAST expression in them
    // - Numbers by themselves are expressions
    // - So the if block returns 5, else block returns 6

    // IMPORTANT: Both arms must return the same type!
    // This would cause an error:
    // let number = if condition { 5 } else { "six" };
    // ^^^ ERROR: if arm returns i32, else arm returns &str

    // Both arms return integers - this is valid:
    let _x = 5;
    let y = {
        let x = 3;
        x + 1 // This expression returns 4
    };
    let number = if y == 4 { 10 } else { 20 };
    println!("Final number: {number}");
}

//
// 4. LOOPS - REPEATING CODE
//

fn loop_example() {
    println!("\n--- LOOP (infinite loop) ---");

    // The 'loop' keyword creates an infinite loop
    // You MUST explicitly break out of it

    let mut counter = 0;

    // This would run forever without the break:
    // loop {
    //     println!("again!");
    // }

    // Break out of a loop with the 'break' keyword
    loop {
        counter += 1;
        println!("Counter: {counter}");

        if counter >= 3 {
            break; // Exit the loop
        }
    }

    println!("Loop finished!");
}

//
// 5. RETURNING VALUES FROM LOOPS
//

fn returning_from_loops() {
    println!("\n--- Returning Values from Loops ---");

    // You can break out of a loop AND return a value
    // Syntax: break value;

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Break AND return counter * 2
        }
    };

    println!("The result is {result}"); // Prints: 20

    // How it works:
    // 1. Loop runs until counter == 10
    // 2. When true, break with value counter * 2 (10 * 2 = 20)
    // 3. The value 20 is assigned to result
    // 4. result is printed
}

//
// 6. BREAK AND CONTINUE
//

fn break_and_continue() {
    println!("\n--- BREAK and CONTINUE ---");

    // break: Exit the current loop immediately
    // continue: Skip remaining code in this iteration, go to next iteration

    for i in 1..=5 {
        if i == 2 {
            println!("{i}: Skipping this one (continue)", i = i);
            continue; // Skip to next iteration
        }

        if i == 4 {
            println!("{i}: Breaking here", i = i);
            break; // Exit loop completely
        }

        println!("{i}: Processing normally", i = i);
    }

    // Output:
    // 1: Processing normally
    // 2: Skipping this one (continue)
    // 3: Processing normally
    // 4: Breaking here
}

//
// 7. LOOP LABELS - Disambiguating Nested Loops
//

fn loop_labels() {
    println!("\n--- Loop Labels (Nested Loops) ---");

    // When you have nested loops, break and continue apply to the INNERMOST loop
    // Use loop labels to control outer loops
    // Labels start with a single quote (')

    let mut count = 0;
    'counting_up: loop {
        // Label for outer loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            // Inner loop (no label)
            println!("  remaining = {remaining}");

            if remaining == 9 {
                break; // Breaks only the inner loop
            }

            if count == 2 {
                break 'counting_up; // Breaks the outer loop!
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
    // Output: End count = 2

    // Note:
    // - First break (no label) exits inner loop only
    // - break 'counting_up; exits the labeled outer loop
    // - The program stops when count reaches 2
}

//
// 8. WHILE LOOPS - Conditional Repetition
//

fn while_loops() {
    println!("\n--- WHILE Loops (Conditional Repetition) ---");

    // while loop: Run code WHILE a condition is true
    // Cleaner than using loop, if, and break together

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Output:
    // 3!
    // 2!
    // 1!
    // LIFTOFF!!!

    // Benefits over loop + break:
    // - More readable: condition is clearly visible
    // - Less nesting required
    // - Intent is immediately clear
}

//
// 9. FOR LOOPS - Iterating Over Collections
//

fn for_loops() {
    println!("\n--- FOR Loops (Collection Iteration) ---");

    // FOR LOOPS are the most common loop in Rust
    // Syntax: for element in collection { }

    // Example 1: Iterate over an array
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Advantages of for loops over while loops:
    // 1. SAFER: No index errors (can't go past array bounds)
    // 2. CLEANER: More concise and readable
    // 3. FASTER: Compiler can optimize better

    // Example 2: Using a Range
    // Ranges: (1..4) means 1, 2, 3 (not including 4)

    println!("\nCountdown using Range:");
    for number in (1..4).rev() {
        // rev() reverses the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // Output:
    // 3!
    // 2!
    // 1!
    // LIFTOFF!!!

    // Other range examples:
    // (1..5)      -> 1, 2, 3, 4 (not including 5)
    // (1..=5)     -> 1, 2, 3, 4, 5 (including 5)
    // (1..5).rev() -> 4, 3, 2, 1
}

//
// 10. COMPARISON: While vs For (With Collections)
//

fn while_vs_for() {
    println!("\n--- While vs For (Array Iteration) ---");

    let array = [10, 20, 30, 40, 50];

    println!("Using WHILE (error-prone):");
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", array[index]);
        index += 1;
    }

    // Problems with this approach:
    // 1. Error-prone: Can forget to update condition if array size changes
    // 2. Runtime checking: Compiler adds bounds-checking code on each iteration
    // 3. More verbose: Need to manage index manually
    // 4. Easy to create off-by-one errors

    println!("\nUsing FOR (idiomatic Rust):");
    for element in array {
        println!("the value is: {element}");
    }

    // Advantages:
    // 1. Safe: Can't go past array bounds
    // 2. Cleaner: Intent is clear
    // 3. Efficient: Compiler can optimize better
    // 4. No manual index management needed

    // CONCLUSION: Use for loops for collections, while for other conditions
}

//
// MAIN FUNCTION - Running Examples
//

fn main() {
    println!("RUST CONTROL FLOW - LEARNING FILE");

    // Run all examples
    if_expressions_example();
    else_if_example();
    if_as_expression();
    loop_example();
    returning_from_loops();
    break_and_continue();
    loop_labels();
    while_loops();
    for_loops();
    while_vs_for();

    println!("All Examples Completed!");
}

//
// SUMMARY OF CONTROL FLOW CONCEPTS
//
/*

    1. IF EXPRESSIONS:
       - Use when you need to branch based on a condition
       - Condition MUST be bool (not int like in C/JavaScript)
       - Can return values from if expressions

    2. ELSE IF:
       - Use for multiple conditions
       - Executes the FIRST matching block
       - Can clutter code with too many branches (use match instead)

    3. LOOPS - Choose based on use case:

       a) LOOP (infinite loop):
          - Use when you need to repeat until a condition is met
          - Must explicitly break out
          - Can return values with break

       b) WHILE (conditional loop):
          - Use when condition-checking is the main logic
          - Clearer than loop + if + break
          - Good for things like "keep looping while this is true"

       c) FOR (iterator loop):
          - MOST COMMON - use this for collections!
          - Safe: Can't go past array bounds
          - Efficient: Compiler optimizes well
          - Works with any iterable (arrays, ranges, etc.)

    4. BREAKING OUT & SKIPPING:
       - break: Exit current loop completely
       - continue: Skip to next iteration
       - Loop labels: Control which loop to break from in nested loops

    5. FOR LOOPS WITH RANGES:
       - (1..5): Generates 1, 2, 3, 4
       - (1..=5): Generates 1, 2, 3, 4, 5
       - (1..5).rev(): Reverses the range

    BEST PRACTICES:
    ✓ Use if for simple branching
    ✓ Use for loops for iterating collections (safer, idiomatic)
    ✓ Use while for condition-based loops
    ✓ Use loop for infinite loops or complex break patterns
    ✓ Use match for many conditions (Chapter 6)

*/
