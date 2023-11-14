use OOP_Design_Pattern::Post;
/*
    Example using the state pattern
    1. A blog post starts as an empty draft.
    2. When the draft is done, a review of the post is requested.
    3. When the post is approved, it gets published.
    4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

*/
fn main() {
    let mut post = Post::new();

    post.add_text("I have a bowl of grapes");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I have a bowl of grapes", post.content());
    
}
