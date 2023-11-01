use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate butter chicken for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    // second approve required
    post.approve();
    assert_eq!("I ate butter chicken for lunch today", post.content());
}
