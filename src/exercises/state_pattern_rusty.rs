#![allow(unused)]

/*
    CHANGES
    - Rather than encapsulating states and transitions
      completely, encode the states into different types.
      This way, Rust's type checking system will prevent
      attempts to use state-exclusive behaviour whilst in
      another state by issuing a compiling error.
      - This means that the transformations between the
        states are no longer encapsulated entirely within
        the implementation of Post.
      - Gain: invalid states are now impossible!
*/

mod blog {
    pub struct Post {
        content: String,
    }
    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    pub struct DraftPost {
        content: String,
    }
    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    pub struct PendingReviewPost {
        content: String,
    }
    impl PendingReviewPost {
        pub fn approve(self) -> PendingSecondReviewPost {
            PendingSecondReviewPost {
                content: self.content,
            }
        }
        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }

    pub struct PendingSecondReviewPost {
        content: String,
    }
    impl PendingSecondReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }
}

pub fn start() {
    use blog::Post;

    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    post.add_text(". It was yummy!");

    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();

    assert_eq!(
        "I ate a salad for lunch today. It was yummy!",
        post.content()
    );
}
