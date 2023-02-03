/** We can define enum data types to imply information between a 
 *  known set of possibilities. Here we define an IpAddrKind enum,
 *  which contains the two possibilities of IP's there are.
 */
enum IpAddrKind {
    V4,
    V6,
}

/** We can use a Struct to define the address of an IP as well as it's
 *  kind. However, we can put data directly into each enum variant.
 *  We can also use different types and ammounts of associated data
 *  for each element of the enum.
 * 
 *  NOTE: IP Address' are such a common data type that Rust has a 
 *  predefined stdlib definition we can use. This are:
 *          struct Ipv4Addr {
 *              // --snip--
 *          }
 * 
 *          struct Ipv6Addr {
 *              // --snip--
 *          } 
 * 
 *          V4(Ipv4Addr),
 *          V6(Ipv6Addr),
 */
enum IpAddr {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

/* Example of enum with multiple data types. */
enum Message {
    // No data associated
    Quit,
    
    // Named fields like a Struct
    Move { x: i32, y: u32},
    
    // Includes a single String
    Write(String),

    // Includes three i32 variables
    ChangeColor(i32, i32, i32),
}

/** Above enum type is similar to the following Struct definitions:
 *          struct QuitMessage;
 *          struct MoveMessage {
 *              x: i32,
 *              y: i32,
 *          }
 *          struct WriteMessage(String);
 *          struct ChangeColor(i32, i32, i32);
 */

 /* We can also use impl to define methods for Enums like we can with Structs */
impl Message {
    fn call(&self) {
        // Method body defined here
    }
}

/** Rust doesn't have a Null value as other languages do. Instead,
 *  it has an Enum that can encode the concept of null of "something"
 *  being present or not. This enum is the Option<T> one and is defined
 *  in the standard library (stdlib) as follows:
 *          enum Option<T> {
 *              None,
 *              Some(T),
 *          }
 *  This enum is used so extensively you don't even need to bring it into
 *  scope. This also aplies to its variants Some and None, which can be
 *  used without the Option:: prefix.
 */

fn main() {
    /* We can create instances of an enum as follows */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    /* Example of the usage of the Option enum. */
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

/** We can now define a function wich takes any instance of IpAddrKind
 *  and does some stuff with it. We can call this function with either
 *  variant of the enum like:
 *          route(IpAddrKind::V4);
 *          route(IpAddrKind::V6);
 */
fn route(ip_kind: IpAddrKind) {}