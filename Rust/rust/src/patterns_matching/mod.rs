/* PATTERNS USE CASES */

/** Match expressions are exhaustive, so all possibilities for the value in the match expression must be accounted. The `_` pattern can be used to match anything,
 *  but it will never bind to a variable, so it's often used in the last match arm.
 */
fn match_use_case(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

/** Here we can see how we can mix and match `if let`, `else if` and `else if let` arms. You can see the different arms don't need to relate to each other.
 */
fn if_let_use_case() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/** In this example, by using the `while let` conditional loop, we run as long as the pattern matches. In other words, we pop values from the stack until it is empty.
 */
fn while_let_use_case() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

/** In a for loop, whatever follows the `for` keyword is a pattern. In this case, we use the `iter().enumerate()` method, which returns a tuple containing the index
 *  and the value requested.
 */
fn for_loops_uses_case() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

/** A let statement also uses patterns because it follows the form `let PATTERN = EXPRESSION;`
 */
fn let_statements_use_case() {
    let x = 5;

    let (w, y, z) = (1, 2, 3);
}

/** Functions use patterns in its parameters as can be seen below.
 */
fn function_parameters_use_case(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

/* Note that patterns can be refutable (will match for any possible value passed) or irrefutable (can fail to match for some possible value). Function parameters, let
 * statements and for loops can only accept irrefutable patterns, while if let and while let statements accept refutable and irrefutable patterns because they are 
 * intended to handle posible failures.
 */

/** We can use match patterns against literals directly as shown in the function below.
 */
fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/** Because match starts a new scope, variables declared as part of a pettern inside `match` will shadow those with the same name outside of the match construct. Below
 *  there is an example of this happening for the value y, while the value x is not shadowed so if its value was None, the default match arm will execute and show us
 *  that the value for x is None.
 */
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default dase, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

/** We can use the `|` syntax to match multiple values (or operator)
 */
fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/** The `..=` syntax allows us to match to an inclusive range of values. First match arm of the funtcion below is equivalent to `1 | 2 | 3 | 4 | 5`. Note that ranges
 *  are only allowed with numeric or char values.
 */
fn matching_with_dot_dot_equal() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let y = 'c';

    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

/** You can break apart a struct to get the values contained inside like shown below. However, if `a` and `b` were to be called `x` and `y` (as in the struct's fields)
 *  the syntax could be reduced to the second example inside the function. We can use this to check against fields and values in a struct with a match expression.
 */
fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/** You can also destructure enums by using syntax similar to the one used in the function below.
 */
fn destructuring_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NewMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

/** You can match structs more than one level deep as you can see in this function.
 */
fn destructuring_nested_structs_and_enums() {
    let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

/** The example below destructures a `tuple(tuple, Point)` into individual variables.
 */
fn destructuring_structs_and_tuples() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

/** You can use the `_` when a function parameter is not used within the body of the function. 
 */
fn ignore_value_with_underscore(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

/** The values inside a pattern can also be ignored with the use of `_`. We can also use `_` in multiple places within one pattern to ignore particular values, like
 *  the match expression associated with the `numbers` tuple.
 */
fn ignore_parts_of_value_with_nested_underscore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 18, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

/** You can state that a variable is not used by starting its name with `_`. Note that it will bind to values so:
 *          let s = Some(5);
 *          if let Some(_s) = s {
 *              println!("found an integer");
 *          }
 *          println!("{:?}", s);
 *  will not compile because `s` was moved into `_s`.
 */
fn ignore_unused_variable() {
    let _x = 5;
    let y = 10;
}

struct PointThreeDimensional {
    x: i32,
    y: i32,
    z: i32,
}

/** We can ignore the remaining parts of a struct making use of the `..` syntax. With it, we can make the fields after the `x` parameter of the struct be `y: _` and
 *  `z: _` with just using `x, ..`. It will also expand to as many values as it needs to be like shown with the `numbers` tuple. However, this syntax must be 
 *  unambigous, that meaning something like `(.., second, ..)` won't work because Rust doesn't know how many values it should skip before second.
 */
fn ignore_with_dot_dot() {
    let origin = PointThreeDimensional { x: 0, y: 0, z: 0 };

    match origin {
        PointThreeDimensional { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

/** Match guards allow for aditional `if` conditions specified after the pattern in a match arm, that must also match for the arm to be chosen. The condition can use
 *  variables created in the pattern, or use ones declared outside of the match expression. 
 * 
 */
fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

enum Greeting {
    Hello { id: i32 },
}

/** The `@` binding lets us create a variable that holds a value at the same time that we're testing that value for a pattern match. In the example below we test that
 *  the variable is in the range 3..=7 and we are also binding it's value to the variable `id_variable` so we can use it in the code of the arm. We could have called
 *  it id as well. In the second match arm, the arm won't know the value inside the `Greeting::Hello` but will know it's in the range.
*/
fn at_bindings() {
    let msg = Greeting::Hello { id: 5 };

    match msg {
        Greeting::Hello { 
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Greeting::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Greeting::Hello { id } => println!("Found some other id: {}", id),
    }
}