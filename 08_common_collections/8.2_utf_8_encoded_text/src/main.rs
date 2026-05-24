fn main() {
    println!("Learning UTF-8 Encoded Text with Strings in Rust\n");

    creating_strings();
    updating_strings();
    concatenating_strings();
    understanding_string_indexing();
    iterating_over_strings();
    working_with_utf8();
}

fn creating_strings() {
    println!("1. Creating Strings");

    let mut s = String::new();
    println!("Empty string created with String::new(): '{}'", s);

    s.push_str("Hello");
    println!("After push_str: '{}'", s);

    let data = "initial contents";
    let s1 = data.to_string();
    println!("Using to_string() method: '{}'", s1);

    let s2 = "direct literal".to_string();
    println!("to_string() works on literals too: '{}'", s2);

    let s3 = String::from("Using String::from");
    println!("Using String::from(): '{}'", s3);

    println!("String::from and to_string do the same thing\n");
}

fn updating_strings() {
    println!("2. Updating Strings");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("After push_str('bar'): '{}'", s);

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!(
        "push_str() doesn't take ownership - s2 is still valid: '{}'",
        s2
    );

    let mut s = String::from("lo");
    s.push('l');
    println!("Using push() to add single char: '{}'", s);

    println!();
}

fn concatenating_strings() {
    println!("3. Concatenating Strings");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("Using + operator: '{}'", s3);

    println!("Important: s1 has been moved and is no longer valid after +");
    println!("s2 is still valid because we used &s2 (reference)\n");

    println!("The + operator uses the add() method which:");
    println!("- Takes ownership of self (left side string)");
    println!("- Takes &str as parameter (we use &String which coerces to &str)");
    println!("- Returns a new String\n");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Multiple + operations: '{}'", s);
    println!("With multiple strings, + gets hard to read\n");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("Using format! macro is cleaner: '{}'", s);
    println!("format! doesn't take ownership of its parameters\n");
}

fn understanding_string_indexing() {
    println!("4. Understanding String Indexing");

    println!("In Rust, you CANNOT index into a String with a single integer:");
    println!("let s = String::from(\"hi\");");
    println!("let h = s[0];  <- This will NOT compile!\n");

    println!("Why? Because strings are more complex than they seem:");
    println!("- Strings are stored as Vec<u8> (vector of bytes)");
    println!("- Different characters take different numbers of bytes");
    println!("- A single index might land in the middle of a multi-byte character\n");

    let hello = "Hola";
    println!(
        "String: '{}' has length {} (4 ASCII chars = 4 bytes)",
        hello,
        hello.len()
    );

    let hello = "Здравствуйте";
    println!(
        "String: '{}' has length {} (12 chars but each uses 2 bytes)",
        hello,
        hello.len()
    );
    println!("Each Cyrillic character needs 2 bytes in UTF-8\n");

    println!("Instead of indexing with [], use ranges for slicing:");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("&hello[0..4] gives us the first 4 bytes: '{}'", s);
    println!("This represents the first 2 Cyrillic characters\n");

    println!("Rust will panic if you try to slice in the middle of a character:");
    println!("&hello[0..1] would panic because 1 is not a char boundary\n");
}

fn iterating_over_strings() {
    println!("5. Iterating Over Strings");

    println!("Use .chars() to iterate over Unicode scalar values:");
    for c in "Зд".chars() {
        println!("  char: '{}'", c);
    }
    println!();

    println!("Use .bytes() to iterate over raw bytes:");
    for b in "Зд".bytes() {
        println!("  byte: {}", b);
    }
    println!("These 4 bytes represent 2 Cyrillic characters\n");

    println!("chars() is usually what you want when working with characters");
    println!("bytes() is useful when you need to work with raw byte data\n");
}

fn working_with_utf8() {
    println!("6. Working with UTF-8 Strings");

    println!("Rust Strings support any properly encoded UTF-8 data:");

    let hello = String::from("السلام عليكم");
    println!("Arabic: {}", hello);

    let hello = String::from("Dobrý den");
    println!("Czech: {}", hello);

    let hello = String::from("שלום");
    println!("Hebrew: {}", hello);

    let hello = String::from("नमस्ते");
    println!("Hindi: {}", hello);

    let hello = String::from("こんにちは");
    println!("Japanese: {}", hello);

    let hello = String::from("안녕하세요");
    println!("Korean: {}", hello);

    let hello = String::from("你好");
    println!("Chinese: {}", hello);

    println!();

    println!("String representation varies by language:");
    println!("The Hindi word 'नमस्ते' is stored as:");
    let hindi = "नमस्ते";
    print!("  Bytes: [");
    for (i, b) in hindi.bytes().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", b);
    }
    println!("]");
    println!("  Total: {} bytes", hindi.len());

    print!("  Chars: [");
    for (i, c) in hindi.chars().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("'{}'", c);
    }
    println!("]");
    println!("  Total: {} characters", hindi.chars().count());

    println!();
    println!("Key insight: 18 bytes become 6 chars (including diacritics)");
    println!("But humans would read this as 4 letters (grapheme clusters)");
    println!("This is why string handling needs care in UTF-8 languages\n");
}
