#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
    Hello { id: i32 },
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("matched literal one"),
        2 => println!("matched literal two"),
        3 => println!("matched literal three"),
        _ => println!("matched the fallback"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("matched outer y: {n}"),
        Some(n) => println!("matched inner value: {n}"),
        None => println!("no value"),
    }

    println!("end state: x = {x:?}, y = {y}");
}

fn matching_multiple_patterns_and_ranges() {
    let x = 1;
    let letter = 'c';

    match x {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three through five"),
        _ => println!("something else"),
    }

    match letter {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("not a lowercase ASCII letter"),
    }
}

fn destructuring_structs_enums_and_tuples() {
    let point = Point { x: 0, y: 7, z: -3 };

    let Point { x, y, .. } = point;
    println!("point fields: x = {x}, y = {y}");

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::Write(text) => println!("text: {text}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("rgb color: {r}, {g}, {b}")
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("hsv color: {h}, {s}, {v}")
        }
        Message::Hello { id } => println!("hello id: {id}"),
    }

    let ((feet, inches), Point { x, y, .. }) = ((3, 10), Point { x: 3, y: -10, z: 0 });
    println!("mixed destructuring: {feet} feet, {inches} inches, point=({x}, {y})");
}

fn ignoring_values_and_match_guards() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("cannot overwrite an existing value"),
        _ => setting_value = new_setting_value,
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("first = {first}, last = {last}"),
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => println!("id in range: {id}"),
        Message::Hello { id } => println!("id outside range: {id}"),
        _ => println!("other message"),
    }
}

fn main() {
    // Each function demonstrates one cluster of pattern syntax ideas.
    matching_literals();
    matching_named_variables();
    matching_multiple_patterns_and_ranges();
    destructuring_structs_enums_and_tuples();
    ignoring_values_and_match_guards();
}
