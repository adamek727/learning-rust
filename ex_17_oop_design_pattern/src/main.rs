extern crate ex_17_oop_design_pattern;

use ex_17_oop_design_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Text 1");
    assert_eq!("", post.content());
    println!("a: {}", post.content());

    post.request_review();
    assert_eq!("", post.content());
    println!("b: {}", post.content());

    post.approve();
    assert_eq!("Text 1", post.content());
    println!("c: {}", post.content());

}