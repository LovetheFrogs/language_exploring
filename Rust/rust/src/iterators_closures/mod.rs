use std::{thread, vec};
use std::time::Duration;

/* CLOSURES */

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /* Method giveaway uses a closure inside the `unwrap_or_else` call. This call takes an argument as a closure 
         * without any arguments on it that returns a value T (same as the one stored in the Option<T>). If the argument
         * is None, then unwrap_or_else calls the closure and returns its value. 
         * 
         * This closure is called after the two vertical bars `||`. If it took any arguments, they would appear between
         * the two bars. Note that unlike methods, functions can't capture their enviroment like this call.
         */
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

/** Closures don't need type anotations, as the compiler can infer it's type in MOST cases. We can add them if we want as seen
 *  in the example below. Closures are short and relevant only within a narrow context rather than in any scenario. They are also
 *  not exposed in an interface, as they are stored in variables and used without naming them.
 */
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

/** Closures can capture values in one of three ways, borrowing immutably, borrowing mutably and taking ownership. Below code 
 *  shows all three of them in use, each one in a separate function.
 * 
 *  Because we can have multiple immutable references to list at the same time, list is still accesible in all the calls to 
 *  println!().
 */
fn immutable_borrow() {
    let list = vec![1, 2, 3];
    // Will print -> Before defining closure: [1, 2, 3]
    println!("Before defining closure: {:?}", list);

    // Will print -> From closure: [1, 2, 3]
    let only_borrows = || println!("From closure: {:?}", list);

    // Will print -> Before calling closure: [1, 2, 3]
    println!("Before calling closure: {:?}", list);
    only_borrows();
    // Will also print -> After calling closure: [1, 2, 3]
    println!("After calling closure: {:?}", list);
}

/** Now, there is no longer a println! between the definition and the call of the closure, when it is defined, it captures a 
 *  mutable reference to list. We don't use the closure after calling it, so the borrow ends. We can't have the removed print 
 *  because we can't have both a mutable and immutable borrow at the same time.
 */
fn mutable_borrow() {
    let mut list = vec![1, 2, 3];
    // Will print -> Before defining closure: [1, 2, 3]
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    
    borrows_mutably();
    // Will print -> After calling closure: [1, 2, 3, 7]
    println!("After calling closure: {:?}", list);
}

/** To force the closure taking ownership of a value, you can use the `move` keyword before the parameter list. This is mostly used
 *  when passing a closure to a new thread so that the data is owned by it. The example below spawn a new thread and gives it a closure
 *  to run as an argument. The closure prints the list, which works with an immutable reference, but because it is called from a thread,
 *  giving ownership to it is neccesary in case the main thread finished first and dropped list.
 */
fn ownership() {
    let list = vec![1, 2, 3];
    // Will print -> Before defining closure: [1, 2, 3]
    println!("Before defining closure: {:?}", list);

    // Will print -> From thread: [1, 2, 3]
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/** The sort_by_key method uses the FnMut trait. This is because it is called more than once, one time for each item in the slice.
 *  The closure `|r| r.width` doesn't capture, mutate or move out anything from its enviroment. Closures like
 *          let mut sort_operations = vec![];
 *          let value = String:.from("by key called");
 * 
 *          list.sort_by_key(|r| {
 *              sort_operations.push(value);
 *              r.width
 *          });
 *  won't work, because it implements just the FnOnce trait. The first time value is pushed, its ownership is transfered to the push
 *  method, making this closure imposible to call multiple times. The correct way to implement a counter for the number of times the
 *  closure is called follows:
 *          let mut num_sort_operations = 0;
 *          
 *          list.sort_by_key(|r| {
 *              num_sort_operations += 1;
 *              r.width
 *          });
 */
fn sort_by_key() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

/* ITERATORS */

/** Function below uses an iterator to print out all of the values in the vector. A for loop internally creates an iterator.
 */
fn vector_iterator() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

/** Test below shows how the iterator next() method works */
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

/** Some methods of the Iterator trait use the `next()` method. This is why you always need to implement this method. Some of 
 *  these methods consume the iterator, like the one tested below (sum() method). Methods that use next are called consuming 
 *  adaptors, because calling them uses up the iterator. Here we cam't use v1_iter after calling its sum method because it 
 *  takes ownership of the iterator.
 */
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

/** Iterator methods that return a new iterator are called iterator adaptors. In the test below, the call to the map() method takes
 *  a closure as an argument. It returns a new iterator that has the result of adding one to the values of the v1 vector. However,
 *  Rust iterators are lazy, so no operation is done until a consuming adaptor method is called, which is why we call the collect() 
 *  method afterwards. It consumes the iterator and collects the values into a collection data type.
 */
#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

/** The code below uses a closure that capture its enviroment. The shoes_in_size() function takes ownership of a vector of shoes and a 
 *  shoe size as parameters. It returns a vector containing shoes of the specified size. In the body of the function, we call into_iter
 *  to create an iterator that takes ownership of the vector. Then we call filter to adapt that iterator into a new iterator taht only
 *  contains shoes of the desired size. The closure captures shoe_size from the enviroment and compares the value with each shoe's size.
 *  Finally, the call to the collect function turns the iterator into a vector, which is returned. 
 */
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}

fn main() {
    /* With type anotation, closures look pretty much like functions. In the example below, you can see the similarities between
     * functions and closures, as well as how much is optional to state with closures. Note that after using a closure with a 
     * certain type, the compiler will "lock" it to the closure, making code like this:
     *          let example_closure = |x| x;
     *          
     *          let s = example_closure(String::from("hello"));
     *          let n = example_closure(5);
     * not compile, as the compiler will infer the closure types to be `String`, thus failing when calling it with an u32.
     */
    fn  add_one_v1                 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|        { x + 1 };
    let add_one_v4 = |x|          x + 1 ;

    // This two lines are needed for the compiler to infer the types of the closures above.
    let mut a = add_one_v3(1);
    a = add_one_v4(3);

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}