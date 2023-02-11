/** Generics in Rust work similarly to the Java ones. Here you can see to functions that differ only in the types of their arguments. They will later be re-written
 *  using generics to avoid code repetition.
 */
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/** We read the following function (same as the two above but defined with generics) as "the function largest is generic over some type T". But not all types can be
 *  compared. Thats why we have to restrict our T by using ': std::cmp::PartialOrd'.
 */
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/** We can also use generics for structs using the <> syntax as follows below.
 */
struct Point<T> {
    x: T,
    y: T,
}

/** Generics can also be used in method declarations, as in functions like this:
 */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/** Methods for only an instance of the generic type can also be defined:
 */
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/** We can use more than one generic type like follows.
 */
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<T, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// Generics can also be used in enums. Examples are the Option<T> or the Result<T, E> enums.

fn main() {
    // Usage of the Point struct, defined with generics.
    let integer = Point { x: 5 , y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let integer_and_float = Point2 { x: 5, y: 4.0 };

    /* Note that both fileds must be of the same type if there is only one generic, as in the Point struct. Lines like
     *          let some_var = Point { x: 5, y: 4.0 }
     * won't work, because x and y are not of the same type.
     */
    
    // Usage of Point2's struct mixup function. Print will show p3.x = 10, p3.y = c.
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}