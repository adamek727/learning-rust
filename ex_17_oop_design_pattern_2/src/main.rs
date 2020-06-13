extern crate ex_17_oop_design_pattern_2;

use ex_17_oop_design_pattern_2::{Post, PendingPost, DraftPost};


fn main() {
    let mut post = Post::new();
    post.add_text("Bublebum");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!(post.content(), "Bublebum")
}
