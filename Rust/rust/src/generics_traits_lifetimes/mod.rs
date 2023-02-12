use std::fmt::{Display, Debug};

/* GENERICS */

/** Generics in Rust work similarly to the Java ones. Here you can see to functions that differ only in the types of their arguments. They will later be re-written
 *  using generics to avoid code repetition.
 */
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/** We read the following function (same as the two above but defined with generics) as "the function largest is generic over some type T". But not all types can be
 *  compared. Thats why we have to restrict our T by using ': std::cmp::PartialOrd'.
 */
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/** We can also use generics for structs using the <> syntax as follows below.
 */
struct Point<T> {
    x: T,
    y: T,
}

/** Generics can also be used in method declarations, as in functions like this:
 */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/** Methods for only an instance of the generic type can also be defined:
 */
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/** We can use more than one generic type like follows.
 */
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<T, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// Generics can also be used in enums. Examples are the Option<T> or the Result<T, E> enums.

/* TRAITS */

/** Traits are similar to interfaces in java. They are methods that can be implemented by different Structs or 
 *  custom data types. They are defined as it follows. They can look similar to function headers in C .h files. 
 *  This trait can be implemented differently by the types/structs which have this trait. Note that in order to
 *  implement a trait on a custom type, either the type or the trait must be defined within the current file. For
 *  example, you can't implement the Display trait on Vec<T>, but you can implement the Sumarize trait to Vec<T>,
 *  or the Display trait on the Tweet type.
 */
pub trait Summary {
    /** Functions here can also have a default behaviour. The default behaviour is used by coding an empty impl block.
     *  Default implementations in traits can also call other methods in the same trait, even if they don't have a 
     *  default implementation yet.
     */
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    /** This method must be defined on every type that implements the Summary trait, while the summarize method doesn't
     *  need to be implemented.
     */
    fn summarize_author(&self) -> String;
}

/** Folowing there are two types (Structs) which implement the Summary trait. */
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/** We can pass traits as parameters to functions. This allows the function to be called with any type that implements
 *  the trait as a parameter. Doing so looks like this:
 */
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/** The impl trait syntax is syntax sugar for a longer form known as a trait bound. The code snipped below shows how it
 *  looks. This is mandatory to use in cases where we want two parameters to be of the same type AND implement a certain
 *  trait:
 *          pub fn notify<T: Summary>(item1: &T, item2: &T)
 */
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/** Multiple trait bounds can be specified in one of these two ways that follow. */
pub fn some_function(item: &(impl Summary + Display)) {}

pub fn other_function<T: Summary + Display>(item: &T) {}

/** In cases where we have a lot of generics and traits, trait bounds make the function signature hard to read, so Rust 
 *  offers an alternative syntax. This is made using the where clause, and can transform a signature like this:
 *          fn some_other_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
 *  into something like the snippet following this comment.
 */
fn some_other_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 3 }

/** We can also use the impl Trait syntax in the return position of a function as follows. Note that you can only return 
 *  a single type.
 */
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}

/** Trait bounds within an impl block that uses generics are used to implement methods conditionally for types that implement
 *  the specified traits. Below you'll seee a Pair Struct and its new method, valid for any generic. It also has a method valid
 *  only for generics that implement the Display AND PartialOrd traits. 
 */
struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/** We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait
 *  on any type that satisfies the trait bounds are called blanket implementations. One example can be the ToString trait,
 *  implemented on any type that implements the Display one. This code block used in the std library looks something like
 *  the following:
 *      impl<T: Display> ToString for T {}
 */

/* LIFETIMES */



fn main() {
    // Usage of the Point struct, defined with generics.
    let integer = Point { x: 5 , y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let integer_and_float = Point2 { x: 5, y: 4.0 };

    /* Note that both fileds must be of the same type if there is only one generic, as in the Point struct. Lines like
     *          let some_var = Point { x: 5, y: 4.0 }
     * won't work, because x and y are not of the same type.
     */
    
    // Usage of Point2's struct mixup function. Print will show p3.x = 10, p3.y = c.
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Usage of the sumarize function used with the Tweet Struct.
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };

    // This will print -> 1 new tweet: horse_ebooks: of course, as you probably already know, people.
    println!("1 new tweet: {}", tweet.summarize());

    // Usage of the default implementation of a trait.
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup CHampionship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // This will print -> New article available! (Read more...)
    println!("New article available! {}", article.summarize());

    /* After implementing the summarize_authors method, println above will output -> New article available! 
     * (Read more from Icebubrgh...)
     */
}