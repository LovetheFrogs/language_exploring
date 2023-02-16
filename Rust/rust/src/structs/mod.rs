/* Note that individual fields of a struct cannot be mutable, just the whole of it. */
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/** Tuple Structs are used to define Tuples as Structs. This is useful because it
 *  allows for a tuple type to have a certain name, while not stating the names of
 *  the fields, as this can sometimes be verbose and redundant.
 */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/** You can also define unit-like structs (Structs without any fields). They behave similarly
 *  to (), the unit type. They are useful when you want to implement a trait without any data
 *  on it.
 */
struct AlwaysEqual;

fn structs() {
    let mut user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("username@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newuserexamplemail@example.com");

    /*  Using struct update syntax, one can create a new instance of a struct with fields
     *  common to another one, but changing others. This is done as shown below.
    
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }; commented to avoid conflit with snippet below */

    /*  We can also only declare the fields different to the original struct by following the
     *  syntax of the code snippet below. Note that ..user1 must be last.
     *  After using this syntax, user1 will not be aviable to use, just as it is
     *  the case when using move, because the username field has been moved to user3.
     *  This is because the String data type doesn't implement the Copy trait. We could
     *  avoid this by using new String values for all the String fields inside user3.
     */
    let user3 = User {
        email: String::from("evenmore@example.com"),
        ..user1
    };

    /*  Black and origin are diferent types because they are instances of a diferent struct. You can
     *  still use the syntax black.0 to access the 0th element of the tuple black. You can also destructure
     *  them.
     */
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

/**
 * Due to the function's parameter names and the struct's field names being the same,
 * you can create an instance of the struct user and use field init shorthand syntax
 * to avoid the tediousness of typing username: username. This is done as shown in the
 * build_user function.
 */
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}