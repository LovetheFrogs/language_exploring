use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    /* In Rust, errors normally call the panic! macro wich halt the program and unwinds the stack, freeing up used space. You can also call
     * this macro from within your code by just calling the macro like:
     *          panic!("crash and burn");
     * When running a Rust program, you can use the enviroment variable RUST_BACKTRACE with a value different than 0 to get a backtrace of
     * exactly what happened to cause the error. This can be useful to track down errors like the one produced by the following code.
     */
    let v = vec![1, 2, 3];
    v[99];

    /* In Rust, you can also handle errors with the Result<T, E> enum. In most cases, the error is not serious enough to require the program
     * to stop entirely. We can handle this in a similar way to the Option enum. This way if we obtain a result, we asign it to a variable.
     * In case we don't get one, we use the panic! macro to halt the program.
     */
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    /* The code snippet above has been commented because it is incomplete. The example below takes it and improves it further. In it, if there
     * is an error, it is handled. In case it is a File Not Found error, it creates the file, and if its creation results in another error, the
     * program will panic. If the error is of another type, the program will also panic, but using a different message. This is achieved by making
     * use of the match expresion.
     */
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem when opening the file: {:?}", other_error);
            }
        },
    };

    /* The code above is complete, but it uses a lot of match expresions. This kind of expressions are very useful, but they are pretty much a 
     * primitive. Instead, you can use closures, discussed in another module, which are more concise than the example shown above.
     */
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    /* Instead of using nested match expresions, we can use either the unwrap() or the expect() methods.
     * 
     * The unwrap() method called over a Result enum will unwrap the result to the desired variable if it is of type Ok. If it's of type Result::Err,
     * it will panic!
     * 
     * The expect() method works similarly, but it will also let us choose the message of the panic! call in case it is reached.
     * Examples of both follow below.
     */
    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
 
    /* Main function can't return anything we want, but we can use it to return a Result<(), E>. This is done in this main function. The E in it is 
     * of type Box<dyn Error>> which is a trait object, and will be talked about in another chapter. Using that return value, the code below will
     * work inside the main function. Using it will also make the main function return 0 if the returned value is Ok, and a noncero value if it
     * return an Err. Main can return any type that implement the std::process::Termination trait, which contains a function report that returns
     * an ExitCode.
     */
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

/** You can also propagate an error. That means returning it to the caller. We will be exploring different ways to do this, but we will start with the
 *  simplest one. The caller will be the one who handles what to do with the different values recieved back from the callee.
 */
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/** As error propagation is so common, Rust provides users with the ? operator. This operator works in almost the same way as the match expressions. This means
 *  that if the value is an Ok, the value inside will be returned from this expression and the program will continue. If it as an Err, the Err will be returned
 *  from the whole function as if the return keyword was used. Note that the ? operator can only be used in functions whose return type is compatible with the 
 *  value the ? is used on. ? can only be used in functions that return Option, Result or a type that implements FromResidual.
 */
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

/** In this example, ? is used in a function which returns an Option<T>. It works similar to using it on a function that resturns Result<T, E>. None will work as
 *  Err does and Some will work like Ok. The ? Operator will not convert a Result into an Option or vice versa.
 */
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}