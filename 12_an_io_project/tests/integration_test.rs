use minigrep::{search, search_case_insensitive};

#[test]
/// Test that verifies case-sensitive search works correctly.
fn case_sensitive() {
    let query = "logic";
    let contents = "\
To the One I Haven't Met,

I spend most of my days looking for logic, solving problems,
and building worlds out of syntax and strings.
But lately, I've realized that no matter how optimized my life is,
there is a persistent gap in the documentation.";

    assert_eq!(
        vec!["I spend most of my days looking for logic, solving problems,"],
        search(query, contents)
    );
}

#[test]
/// Test that verifies case-insensitive search works correctly.
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
