#![allow(unused)]

/*
    TRADE-OFFS OF STATE PATTERN: UPSIDES
    - Methods on Post know nothing about various behaviours.
    - Look at State trait on Published stuct to see the diff
      ways a published post can behave
    - Without state pattern, would need match expressions in
      methods of Post or even in main code that checks state
      of post & changes behaviour in those places. Each of
      those match expressions would need another arm for every
      new state.
    - With state pattern, no match expressions needed.
    - To add a new state, only have to add a new struct and
      implement the trait methods on that one struct.

    TRADE-OFFS OF STATE PATTERN: DOWNSIDES
    - Since the states implement the transitions, some states
      are coupled to each other.
    - Some logic duplicated. Can't make default impls for
      request_review and approve because trait doesn't know
      exactly what concrete self will be. To use State as
      trait object, methods must be object safe.
    - Similar request_review and approve methods on Post.
      Could define macro to eliminate repetition.
    - This particular implementation is more similar to
      traditional OOP language implementations. Can use a
      more Rust-like implementation to catch more errors at
      compile time.
*/

mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            // Any new Post starts as a draft
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            // // not part of state pattern; behaviour doesn't
            // // depend on state of Post
            // self.content.push_str(text);

            // only allowed to change text content when post
            // in Draft state
            if let Some(s) = self.state.take() {
                self.content.push_str(s.add_text(text));
                self.state = Some(s);
            }
        }

        pub fn content(&self) -> &str {
            // delegate to content method defined on its
            // state.

            // use as_ref because we want a reference to
            // the value inside Option, not ownership.

            // when we call unwrap, we know that None is
            // impossible because methods on Post ensure
            // that state will always contain a Some value
            // when those methods are done.
            self.state.as_ref().unwrap().content(self)
        }

        // requesting review of Post changes State
        // method is the name, no matter the state value.
        // each state is responsible for its own rules.
        pub fn request_review(&mut self) {
            // call take to take Some and leave None in its
            // place
            if let Some(s) = self.state.take() {
                // set Post's state to result
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        // consumes the current state and returns the new state
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        // need lifetime annotations: taking ref to a post
        // as an arg and returning a ref to part of that post,
        // so the lifetime of the returned ref is related to
        // the lifetime of that post arg.
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn add_text<'a>(&self, str: &'a str) -> &'a str {
            ""
        }
    }
    struct Draft {}
    impl State for Draft {
        // change state: Draft -> PendingReview
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        // returning self = no changes
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn add_text<'a>(&self, str: &'a str) -> &'a str {
            &str
        }
    }
    struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        // change state: PendingReview -> Published
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingSecondReview {})
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }
    struct PendingSecondReview {}
    impl State for PendingSecondReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }
    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
}

pub fn start() {
    use blog::Post;

    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.add_text(". It was yummy");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("...NOT!!!!!!!!");
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(
        "I ate a salad for lunch today. It was yummy",
        post.content()
    );
}
