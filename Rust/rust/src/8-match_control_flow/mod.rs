#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Georgia,
    Florida,
    California,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/** A match expression matches the value of a certain enum to one code snipet,
 *  depending on the value of that instance of the enum type. In this case, the
 *  function value_in_cents returns the value in cents of a dollar of each of
 *  the types of coins defined in the coin enum.
 *
 *  You can use curly braces to match each value to more than just an expression.
 *  In that scenario, you don't need to separate that arm from the next one. See
 *  example below for further reference.
 *
 *  NOTE: You must implement all the arms' patterns or your code will not compile.
 */
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        /*  We can check the value of the UsState of a certain quarter by calling the
         *  variable state as shown. Then, we can print out the state as our state
         * variable will hold the value of the UsState of that quarter.
         */
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

/** We can use this match construct to get the value of the inner T when working with
 *  an Option<T>. The following function takes an Option<i32> and if it has a value
 *  that is, if it is not None (null), adds one to that value. If it is None it will
 *  not attempt any operations.
 */
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/* Following functions are used to explain a code snippet below */
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /*  We can use a variable inside a match to catch all values not considered (sorta like
     *  Java's default clause inside switch statements). This variable can have any name if
     *  you want to use it for a function, or an underscore (_) if the value is not used
     *  (like Python's _ in for loops).  You can also ignore all other cases by using the
     *  unit value (). Refer to coode below for demonstration.
     */
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }
}