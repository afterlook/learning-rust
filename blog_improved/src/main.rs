use blog_improved::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate butter chicken for lunch today");

    let post = post.request_review();

    let post = post.approve();

    let post = post.reject();

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    assert_eq!("I ate butter chicken for lunch today", post.content());
}
