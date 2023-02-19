use std::ops::Deref;

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

fn main() {
    /* Demonstration on how to use a Box<T> to store an i32 value on the heap. */
    let b = Box::new(5);
    println!("b = {}", b);

    // Declaring a new List type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // By implementing the Deref trait in the MyBox<T> struct, we can use the * operator to deref a value and thus, the code below will work.
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}