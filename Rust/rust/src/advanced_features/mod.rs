use std::fmt;
use std::slice;
use std::ops::Add;
use std::collections::HashMap;

/* UNSAFE RUST */

/** Raw pointers are a type of pointers that can ignore borrowing rules, aren't guaranteed to point to valid memory, are allowed to be null and don't implement any
 *  automatic cleanup. This pointers can also be immutable (written as `*const T`) or mutable (`written as *mut T`). In the context of raw pointers, immutable means
 *  that the pointer can't be directly assigned to after being dereferenced. Creating this raw pointers is not unsafe per se, it's dereferencing them what's not safe.
 *  For example, the raw pointer `r` in the function below points to a random memory address. When dereferencing it, we don't know if they are pointing to valid memory.
 *  Note that we created a mutable and immutable reference to the same value (`r1` and `r2`), wich is (usually) not suported by the borrow checker. Be careful as raw
 *  pointers can potentially create a data race. Raw pointers are mostly used to interact with C code or when building up safe abstractions that the borrow checker does
 *  not understand.
 */
fn creating_dereferencing__raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

/** Calls to unsafe functions must be done inside unsafe blocks as this tells the compiler we've read the documentation for that function and we understand the risks
 *  of using it and how it's done. The body of an unsafe function is effectively an unsafe block, so there is no need to add another unsafe block.
 */
fn unsafe_functions_and_calls() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

/** A function containing unsafe code doesn't mean that we have to mark the whole function body as unsafe. Wrapping it in a safe function is a common abstraction. We
 *  will implement the `split_at_mut()` function, which has unsafe code, and call it from a safe function to demonstrate this.
 */
fn safe_abstraction_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/** Another use of an unsafe block is calling functions coded in other languages. We must tell the `extern` keyword the language to spect and then type in the function
 *  signature and name. The `"C"` part tells Rust to call the C Application Binary Interface (ABI).
 */
fn call_extern_code() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

/** We can also use extermm to create an interface that allows other languages to call Rust functions. We just add the `extern` keyword and specify the ABI to use
 *  before the fn keyword. We also add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle the function's name. We can then compile the function to
 *  a shared library and linked from C. The usage of `extern` doesn't require `unsafe`.
 */
#[no_mangle]
pub extern "C" fn to_call_from_c() {
    println!("Just called a Rust function from C!");
}

/** Rust's global variables are called static variables. Function below shows example declaration and usage of a static variable. Static variables are similar to
 *  constants, but they always point to the same memory address and using its value will always access the same data, while constants are allowed to duplicate their
 *  data. Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is unsafe.
 */
static HELLO_WORLD: &str = "Hello, world!";

fn use_global_variable() {
    println!("name is: {}", HELLO_WORLD);
}

/** Code below shows how to use and access a mutable static variable. Any code that reads or writes from `COUNTER` must be within an unsafe block.
 */
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn manipulating_static_variable() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

/** A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify. We must add the unsafe keyword before trait and also mark
 *  its implementation as unsafe. An example in where you'd use this is when we implement a type that is not `Send` or `Sync` such as raw pointers and we want to mark
 *  that type as one of both. We therefore need to use the unsafe keyword.
 */
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

/* The final action that works only with unsafe is accessing fields of a union, wich is a structure similar to a struct, but in which only one field is used in a
 *  particular instance at one time. They are primarily used to interface with unions in C code.
 */

/* ADVANCED TRAITS */

/** An advanced feature of traits are Associated types, which connect a type placeholder with a trait such that the trait method definitions can use these placeholder
 *  types in teir signatures. An example of this is the Iterator trait. Associated types seem similar to Generics, but generics make us annotate the types in each 
 *  implementation, while associated types don't let us implement a trait on a type multiple times. Associated types also become part of the trait's contract so
 *  implementors of the trait must provide a type to stand in for the placeholder.
 */
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

/** When using generic type parameters, we can specify a default concrete type for the generic by using the syntax `<PlaceholderType=ConcreteType>`. An example of 
 *  when this is useful is with operator overloading in wich you customize the behaviour of an operator. Code below overrides the `+` operator, but makes use of 
 *  the default for the Add trait. This trait is in fact making use of this, as its signature is:
 *          trait Add<Rhs=Self> {
 *              type Output;
 * 
 *              fn add(self, rhs: Rhs) -> Self::Output;
 *          }
 *  In this case, we used the default value, wich is `Self`, so the type for `rhs` will be Point.
 */
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_generic_type_operator_overloading() {
    assert_eq!(
        Point { x: 1, y: 0} + Point { x: 2, y: 3},
        Point { x: 3, y:3 }
    )
}

/** To override the `+` operator and allow us to add an instance of Millimeters with one of Meters, we specify `impl Add<Meters>` so that the `Rhs` type parameter is
 *  Meters instead of Self. You'll use default type parameters instead of the default of Self to extend a type without breaking existing code or to allow customization
 *  in specific cases.
 */
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

/** You can sometimes have a method with the same name as another trait's method. Rust allows this, but needs to be told which method you want to use. In the code 
 *  below, we've defined two traits with a method that has the same name in both and we've also created a method for Human that also has the same name `fly`. In the
 *  `syntax_for_disabiguation()` function we can see how we can call each method.
 */
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn syntax_for_disambiguation() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

/** The example below shows how we can call trait functions (they don't take self as a parameter) that have the same name. We have to use fully qualified syntax so
 *  Rust knows what function it should call. This syntax has the form:
 *          <Type as Trait>::function(receiver_if_method, next_arg, ...);
 *  You can use this syntax everywhere you call functions or methods, but most of the time, Rust can figure out what function you mean. Note that for functions, the 
 *  `receiver` will not be present.
 */
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn fully_qualified_syntax() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

/** Sometimes you require a type to implement a trait in order to implement that type. Below there's an example of this behaviour where the supertrait `OutlinePrint`
 *  needs the type it is implemented in to also implement the `Display` trait. Our Point struct does not implement it so for it to be able to implement OutlinePrint,
 *  it will also need to implement Display.  
 */
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn supertraits() {
    let p = Point { x: 1, y: 3 }.outline_print();
}

/** In Chapter 10-Implementing a Trait on A Type section of the Book, the orphan rule is mentiones, which says we're only allowed to implement a trait on a type if
 *  either the type or the trait are local to our crate. We can get around this if we wrap the type in a struct as seen below (this is the Newtype pattern, originated
 *  from Haskell). The downside of this technique is that Wrapper is a new type so it must implement all of Vec<T> methods by itself. We could either implement the 
 *  Deref trait to access the inner type or implement manually all the methods of Vec<T> we want to use.
 */
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[test]
fn newtype_pattern_external_traits_and_types() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w: {}", w);
}

/* ADVANCED TYPES */

/** Newtype pattern can also be used to indicate the units of a value and statically enforcing that values are never confused, like in the Meters and Millimiters
 *  structs. We can use it to abstract away some details of a type, like for example the People struct below. This struct will be part of the public API so users don't
 *  know how we encode names. This is a lightweight way to achieve encapsulation to hide implementation.
 */
struct People(HashMap<i32, String>);

/** We can create type synonyms by using the `type` keyword. This will be treated the same as i32 values, so it is ifferent to the Meters/Millimiters structs. Note that
 *  this doesn't have the type-checking advantages using the Newtype pattern with structs does.
 */
fn type_synonyms() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

/** This synonyms behaviour is useful to encode large types so funtcion heads are not so lenghty.
 */
fn synonyms_use_case() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        let a: Thunk = Box::new(|| println!("placeholder"));
        a
    }
}

/** Another usage of aliases is found in the `std::io` module. The return values are mostly of type `Result<T, std::io::Error>`, so an alias is used like shown below.
 */
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

/** The never type `!` means that the function will never return. This functions are called diverging functions and we can't create values of type `!`. Some functions
 *  with the ! return type are continue, panic!, or loop (in case they don't include a break).
 */
fn bar() -> ! {
    loop {
        // --snip--
    }
}

/** Some types are dynamically sized (DST or unsized types). The compiler needs to know how much space does a variable use, so thats why code like:
 *          let s1: str = "Hello there!";
 *          let s2: str = "How's it going?";
 *  doesn't work. The size of s1 and s2 is unknown at compile time. To solve this, we use references or pointers (so s1 and s2 should be &str, not str) that store
 *  metadata. In case of the &str type, the lenght and starting position is stored, so the size of any &str type is known to be 2 * usize. We can combine str wit all
 *  kinds of pointers like Box<str> or Rc<str>. Another kind of DSTs are traits. Recall when we used traits as trait objects and placed them behind Box<dyn Trait> or
 *  &dyn Trait. To work with DSTs, Rust provides the `Sized` trait to determine if a type's size is known at compile time. This trait is automatically implemented for
 *  everything whose size is known at compile time. In addition, Rust implicitly adds a bound to Sized to every generic function. So a function like:
 *          fn generic<T>(t: T) {
 *              // --snip--
 *          }
 *  actually looks like:
 *          fn generic<T: Sized>(t: T) {
 *              // --snip--
 *          }
 *  This is applied by default, so if you want to use a type which doesn't implement the Sized trait, you will use a signature similar to the one of the function
 *  below. Note that the `?` notation is only available for Sized. Also note that the `t` parameter changed from T to &T because it may not be Sized, so we need to
 *  use it from some kind of pointer.
 */
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

/* ADVANCED FUNCTIONS AND CLOSURES */

/** You can use functions as parameters by using the `fn` type. The fn type is NOT a trait, so we need to specify it as the parameter type directly, rather than
 *  creating a generic type parameter with one of the `Fn` traits as trait bounds. You'll usually want functions written with a generic type so it can accept both,
 *  functions and closures. 
 */
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_as_parameter() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

/** There is a situation when you DON'T want to accept closures, and only functions. This is when interacting with external code. For example, C accepts functions as
 *  arguments, but C doesn't have closures. Below you can see an example where both a closure or a function can be used as arguments.  
 */
fn closures_and_functions_as_arguments() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
}

/** We can use enums initializer functions as function pointers as well. Code below takes the u32 values in the range map is called on and creates `Status::Value` 
 *  instances.
 */
enum Status {
    Value(u32),
    Stop,
}
fn enum_initializer_example() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

/** To return a closure, the return type must be inside a pointer because of the Sized trait talked about earlier. This example returns a type which implements the
 *  `Fn` trait, so a closure that implements that trait, or a function can be returned.
 */
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

/* MACROS */

/** The most used form of macros in Rust are declarative macros or macros by example. They allow for something similar to a match expression, as they compare a value
 *  to patterns that are associated with particular code. In this situation, the value is the literal Rust source code passed to the macro and the code associated
 *  with each pattern, when matched, replaces the code passed to the macro. This all happens during compilation. The macro implemented below is similar to the vec!
 *  macro, explained below.
 * 
 *  The `#[macro_export] annotation indicates that this macro should be made aviable whenever the crate in which the the macro is defined is brought into scope. It
 *  is needed to bring the macro into scope. Then, we start the macro definition with `macro_rules!` and the macro name WITHOUT the !. The structure inside the vec
 *  body is similar to the structure of a match expression. We have one arm with the pattern `( $( $x:expr ),* )` followed by `=>` and the code associated with this
 *  pattern.
 * 
 *  This pattern pieces mean:
 *      - A set of parentheses to encompass the whole pattern.
 *      - A dollar sign ($), used to declare a variable in the macro system.
 *      - Another set of parentheses that captures values that match the pattern.
 *      - The `$x:expr` wich matches any Rust expression and gives the expression the name $x.
 *      - The comma which indicates that a literal comma separator character may appear after the code that matches the code in $().
 *      - A * symbol, specifiyng that the pattern matches zero or or more of whatever precedes the *.
 *  
 *  Now, lets look at the pattern in the body of the code associated with this arm.
 *      - temp_vec.push() within $()* is generated for each part that matches $() in the pattern zero or more times.
 *      - $x is replaced with each expression matched.
 *  
 *  When we call this macro with `vec![1, 2, 3]`, the code generated that replaces this macro call will be:
 *          {
 *              let mut temp_vec = Vec::new();
 *              temp_vec.push(1);
 *              temp_vec.push(2);
 *              temp_vec.push(3);
 *              temp_vec
 *          }
 *  This macro can take any number of arguments of any type and can generate code to create a vector containing the specified elements. To learn more about macros,
 *  consult online documentation such as "The little Book of Rust Macros".
 */
#[macro_export]
macro_rules! vec {
    ( $( $x:expr), * ) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    };
}

/** Procedural macros act more like functions and accept some code as input, operate on it and produce some code as an output, rather than matching patterns and
 *  replacing code with other code as declarative macros do. When creating a procedural macro, the definitions must reside in their own crate with a special crate
 *  type. In the example below, `some_attribute` is a placeholder for using a specific macro variety:
 *          extern crate proc_macro;
 *          use proc_macro::TokenStream;
 * 
 *          #[proc_macro_derive(some_attribute)]
 *          pub fn some_name(input: TokenStream) -> TokenStream {
 *              // --snip--
 *          }
 * 
 *  We created a new crate named hello_macro to illustrate this. To see how to implement procedural macros, check that crate. Below you can see the usage of the macro
 *  defined there.
 */
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[test]
fn procedural_macro() {
    Pancakes::hello_macro();
}

/* Attribute-like macros work similarly to procedural macros, but they are more flexible than them, because derive only works for structs an enums. Take for example 
 * the use case:
 *          #[route(GET, "/")]
 *          fn index() { // --snip-- }
 * 
 * This route attribbute annotates functions when using a web application framework. This attribute would be defined as a procedural macro. Its signature would look
 * like:
 *          #[proc_macro_attribute]
 *          pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream { // --snip-- }
 * 
 * Here we have two attributes of type TokenStream. The first is for the contents of the attribute (GET, "/"). The second is the body of the item the attribute is 
 * attached to (fn index() {}).
 */

/* Function-like macros define macros that look like function calls. Similar to `macro_rules!` macros, they are more flexible than functions, for example, they can 
 * take an unknown number of arguments. However, function-like macros take a TokenStream parameter and their definition manipulates that TokenStream as procedural 
 * macros do. An example of a function-like macro is an `sql!` macro thath might be called like:
 *          let sql = sql!(SELECT * FROM posts WHERE id=1);
 * 
 * This macro would parse the SQL statement inside and check that it's syntactically correct, wich is much more complex processing than a `macro_rules!` macro can do.
 * The `sql!` macro would be defined like:
 *          #[proc_macro]
 *          pub fn sql(input:TokenStream) -> TokenStream { // --snip-- }
 * 
 * This definition is similar to the custom derive macro's signature.
 */
