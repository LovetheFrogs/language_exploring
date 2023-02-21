/** Fields of `AveragedCollection` are not marked asa pub so that users are forced to use the methods defined for the struct and thus, the average is always updated.
 */
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

/** GUI library to show how polymorphism works in Rust making use of Enums and Traits.
 */
mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // Box<dyn Draw> ensures all the types inside the components vector implement the Draw trait.
        pub components: Vec<Box<dyn Draw>>
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }
}

/* Posible usage of the GUI library created above. 
 */
use crate::object_oriented_rust::gui::{Draw, Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

/** Here you can see how the state design pattern can be implemented in Rust. For more information about this module, check The Book, Chapter 17.3, as this is just 
 *  a way to implement a design pattern in Rust.
 */
mod implementing_design_pattern {
    mod blog {
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }

            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }

            pub fn content(&self) -> &str {
                self.state.as_ref().unwrap().content(self)
            }

            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }
        }

        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }
            fn reject(self: Box<Self>) -> Box<dyn State>;
        }

        struct Draft {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        struct PendingReview {}

        impl State for PendingReview {
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
    
    use blog::Post;
    
    #[test]
    fn main() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

/** Here is another way of implementing the module above without the OO work-alikes used in the other module. Here we take advantage of Rust's strenghts because, for
 *  example, it ensures there is no way to access the contents of an unpublished post. Also, this solution is a bit more efficient than the other one, while having a
 *  bit less flexibility than Dynamic Dispatch. 
 */
mod implementing_design_pattern_2 {
    mod blog {
        pub struct Post {
            content: String,
        }

        pub struct DraftPost {
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
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }
    }
    
    use blog::Post;

    #[test]
    fn main() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        
        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}