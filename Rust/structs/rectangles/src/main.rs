/** To make use of structs, we will write a program that calculates the area of a rectangle,
 *  and then refactor the program to use structs.
 */


/** Program without any Structs or Tuples 
 * 
 *  fn main() {
 *      let width1 = 30;
 *      let height1 = 50;
 * 
 *      println!(
 *          "The area of the rectangle is {} square pixels.",
 *          area(width1, height1)
 *      );
 *  }
 * 
 *  fn area(width: u32, height: u32) -> u32 {
 *      width * height
 *  }
*/

/** Same program using Tuples
 * 
 *  fn main() {
 *      let rect1 = (30, 50);
 * 
 *      println!(
 *          "The area of the rectangle is {} square pixels.",
 *          area(rect1)
 *      );
 *  }
 * 
 *  fn area(dimensions: (u32, u32)) -> u32 {
 *      dimensions.0 * dimensions.1
 *  }
 */

/** Same program but using Structs. It is more useful than tuples as
 *  we can specify the name of the fields thile simplifying the writing
 *  of the code.
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/** Methods are similar to functions, but they are defined within the 
 *  Struct's context. Their first parameter is always self (the instance
 *  of the Struct the method is being called on).
 * 
 *  To define the methods of a Struct, we start by using the impl block for
 *  Rectangle (the Struct's name). This is similar to Java's Class methods 
 *  for a certain class.
 */
impl Rectangle {
    /** We use &self instead of self as method's parameter because we don't
     *  want the area method to take ownership of the Struct.
     */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    /** Method can_hold takes an extra Rectangle Struct and tells if it fits
     *  inside the original rectangle (the one used to call this method). It uses
     *  &Rectangle to avoid taking ownership of the other rectangle.
     */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /** Associated functions don't take self as an argument. This are functions
     *  like the String's type ::from function. They are often used for constructors
     *  and are usually called new, but new is not a special name in Rust.
     */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    /** While using :? inside the {} structure, and using the tag 
     *  #[derive(Debug)] at the start of the code, we can use the Debug
     *  printing format (useful for developing).
     * 
     *  This format looks like thhis:
     *          Rectangle { width: 30, height: 50 }
     * 
     *  If the tag used is :#? instead, output will be more readable:
     *          Rectangle {
     *              width: 30,
     *              height: 50,
     *          }
     *
     *  You can use more traits such as #[derive(Debug)]. See Rust's Apendix C
     *  for more info. You can also implement your own traits.
    */
    println!("rect1 is {:#?}", rect1);

    /** We can also use the dbg! macro to print values using the Debug format.
     *  Note that, opposed to println!, dbg! takes ownership of an expression,
     *  while println! takes a reference. Also, dbg! prints to stderr, while
     *  println! outputs to stdout.
     */
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    /* Reference passed because we don't want dbg! to take ownership of rect2. */
    dbg!(&rect2);

    println!(
        "The area of the rectagle is {} square pixels.",
        rect2.area()
    );

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /* To use associated functions, we must use the :: syntax. */
    let sq = Rectangle::square(3);

    dbg!(&sq);
}