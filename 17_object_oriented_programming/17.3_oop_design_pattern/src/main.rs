use oop_design_pattern::Post;

fn main() {
    let mut post = Post::new();

    println!("1. New draft created");
    post.add_text("I ate a salad for lunch today");
    println!("Draft content: {:?}", post.content());
    assert_eq!("", post.content());

    println!("2. Requesting review");
    post.request_review();
    println!("Review content: {:?}", post.content());
    assert_eq!("", post.content());

    println!("3. Approving post");
    post.approve();
    println!("Published content: {:?}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());
}
