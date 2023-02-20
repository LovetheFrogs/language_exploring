use std::ops::Deref;
use std::rc::Rc;

/* Box<T> Smart Pointer */

/* This List enum comes from the `Cons List` implemented in functional languages such as Lisp. They are an issue because its size is unknown at compile time.
 * As such, it is an scenario where you'd want to use the Box<T> smart pointer. Cons Lists are made up of pairs. That means that each pair contains its own 
 * element and the next one. The example below ilustrates this:
 *          (1, (2, (3, Nil)))
 *          ^----------------^ First tuple,  elem_1 = 1 | elem_2 = (2, (3, Nil))
 *              ^-----------^  Second tuple, elem_1 = 2 | elem_2 = (3, Nil)
 *                  ^------^   Third tuple,  elem_1 = 3 | elem_2 = Nil
 * The unknown size at compile time makes the use of a Box<T> needed for the enum below to compile, as then a reference will be stored in the stack and the
 * final size of a List instance won't be needed to know at compile time.
 */
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::smart_pointers::List::{Cons, Nil};

/* The Deref trait */

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/** Rust's Deref coercion lets programmers wrote function and method calls without needing to add explicit references and dereferences (& and *). This function and
 *  its test shows this Rust feature in action. NOTE: Deref coercion also works in mutable references by using the DerefMut trait. Rust does deref coercion in three
 *  cases:
 *          From &T     to &U     when T: Deref<Targer=U>
 *          From &mut T to &mut U when T: DerefMut<Tager=U>         (If you have T wich implements Dered/DerefMut on some U, you can get &U)
 *          From &mut T to &U     when T: Deref<Target=U>
 *  The third case allows programmers to coerce a mutable reference to an immutable one. This does not work the other way because it can only be done if there is only
 *  one mutable reference, and since it can't be known at compile time, it can go againts Rust's borrowing rules.
 */
fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[test]
fn deref_mybox() {
    // By implementing the Deref trait in the MyBox<T> struct, we can use the * operator to deref a value and thus, the code below will work.
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    /* If Rust had no deref coercion, this line would be the correct function call, where:
     *          *    -> dereferences String from MyBox<String> 
     *          [..] -> selects the whole String for the slice
     *          &    -> makes a String slice from the selected part of the String
     */
    hello(&(*m)[..]);
}

/* The Drop trait */

/** By implementing the Drop trait on any data type, one can make Rust use that code whenever a variable is dropped. The `drop()` function takes a mutable reference to 
 *  self
 */
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/** At the end of this test function, the variables c and d will go out of scope and so, Rust will call the drop method on them. Output should be:
 *          CustomSmartPointers created.
 *          Dropping CustomSmartPointer with data `other stuff`!
 *          Dropping CustomSmartPointer with data `my stuff`!
 *  Here you can see that Rust drops variables in reverse order of creation (from top of stack to bottom).
 *  Sometimes (like when working with locks) you'd want to drop a variable before it goes out of scope. You can do this by calling the std::mem::drop function, as the drop
 *  method of a smart pointer can't be called manually.
 */
#[test]
fn drom_smart_pointer() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer c dropped before end of scope.");
}

/* Rc<T> (Reference Counted) Smart Pointer */

/** By using a Rc<T> Smart Pointer instead of a Box<T> one, we can make the data stored shared by two or more variables. See main function for an example on why this might be 
 *  useful. Another example will be the graph data structure, where a node is owned by some edges, and that node is valid until pointed by no edges. We can then say that the 
 *  node is "owned" by the edges.
 */
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

/** Test here should be in main but placed inside a test for ease of running. It shows the number of reference counts changing. When the test is exited, the number of owners is
 *  0 and thus, the Drop::drop method is called to free up the variable.
 */
#[test]
fn rc_count() {
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

/* RefCell<T> Smart Pointer */

/** Implemented below is a library to test with the following elements:
 *      - A trait `Messenger` with a method in it that sends a message
 *      - A struct `LimitTracker` with three fields, the value of a message, the max lenght and the messanger (wich must implement the Messager trait).
 *      - An implementation for the struct LimitTracker with the methods:
 *          * new       -> creates a new instance of the LimitTracker struct.
 *          * set_value -> sets a value and depending on its relation to the max field makes the messager send a message.
 */
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { 
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of ypur quota!");
        }
    }
}

/** Below are the tests for the library implemented on lines 151 to 186. In this test we will create a Mock Object, explained later in this comment. This
 *  tests have the following elements:
 *      - A `MockMessenger` struct, that keeps track of the messages sent inside a RefCell<T> smart pointer wich contains a vector of Strings.
 *      - An implementation for the `new` method of this struct, wich creates a new instance of RefCell<T>.
 *      - An implementation of the `Messenger` trait that takes a mutable reference of the vector stored inside the RefCell<T> and pushes to it a message.
 *      - A test that SHOULD send a message warning us of an over 75% usage of the max capacity.
 *  In order to test that the message is sent, we used what is called a Mock Object. This kind of objects are kind of a "second actor" used to check wether
 *  or not our library is implemented correctly. In order to follow the Messenger trait and also avoid the usage of lifetimes, this Mock Object (MockMessenger
 *  from now on) will store a RefCell<T> containing a vector of Strings. This is to allow changing the values of the vector without having to make it mutable 
 *  (wich goes against the trait's signature).
 * 
 *  Now, to mutate the vector, the `borrow_mut()` method of RefCell<T> is called, wich gives us a mutable reference to the value inside the RefCell<Vec<String>>.
 *  In order to get an immutable reference, we just have to call the `borrow()` method.
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/** Using a RefCell<T> Smart Pointer doesn't allow us to fully bypass the borrow rules. The following test is similar to the last one, but it changes the 
 *  implementation of the `Messenger::send()` method so that it creates two mutable references. This code will compile, but it will panic! at runtime, that's 
 *  why the test is also annotated with the `#[should_panic]` tag, as it creates two mutable references within the same scope.
 *  
 *  The RefCell<T> borrow method returns an instance of the smart pointer type Ref<T>, while the  borrow_mut one returns an instance of the smart pointer RefMut<T>.
 *  RefCell<T> just keeps track of the number of mutable and immutable references so that it knows when to panic!. 
 */
#[cfg(test)]
mod panic_tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn should_panic_on_multiple_mutable_references() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
    }
}

/** Following module shows how Rc<RefCell<T>> will work together to have variables with multiple owners that can also be mutable. Note that RefCell<T> does not work 
 *  for multithreaded code. Instead, Mutex<T> (thread-safe version) should be used.
 */
mod list_with_refcell_and_rc {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    #[test]
    // We can combine a RefCell<T> inside a Rc<T> to allow sharing a variable withing various owners while also making it mutable.
    fn cons_list_refcell_rc() {
        use crate::smart_pointers::list_with_refcell_and_rc::List::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

/* Reference Cycles and Memory Leaks */

mod memory_leaks {
    use crate::smart_pointers::memory_leaks::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    /** This test creates a reference cycle by first linking the end of `b` to `a` and then doing the same with `a`. Due to this cycle, the rc count of both a and b
     *  will decrease to 1 after the program ends, but as it is never 0, the values won't be dropped.
     */
    #[test]
    fn stack_overflow_cycle() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc vount = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncommenting the following line will cause a StackOverflow due to a cycle being present.
        // println!("a next item = {:?}", a.tail());
    }

}

mod tree {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    /** The parent of a node is of type Weak<T>, because we dont't want children to drop parents when they are drop and also we need to avoid creating cycles. To
     *  create a new Weak<T> instance from a Rc<T>, call `Rc::downgrade(&var)`. To check if a Weak<T> still exists, you can call `Weak.upgrade()` which returns an 
     *  Option<T>, containing the value in case it exists or None otherwise.
     */
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

use crate::smart_pointers::List2::{Cons2, Nil2};

fn main() {
    /* Demonstration on how to use a Box<T> to store an i32 value on the heap. */
    let b = Box::new(5);
    println!("b = {}", b);

    // Declaring a new List type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Note that the Rc::clone method DOES NOT make a copy of the variable, it just increments the reference count
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    let b = Cons2(3, Rc::clone(&a));
    let c = Cons2(4, Rc::clone(&a));
}