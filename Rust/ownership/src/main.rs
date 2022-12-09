fn main() {
    // We declare a String literal s
    let s = "hello";

    {   
        // Shadow literal s inside the scope of the brackets
        let s = "Hello";

    }   // Now s = hello again

    // We re-define s as a mutable String from a string literal. This allows us to do operations with it
    let mut s = String::from("hello");

    // Use of String.push_str() to append one string to another
    s.push_str(", world!");

    println!("{}", s);

    // drop() function can be used to unallocate space used by Strings or other data types (same as free() in C)

    // This creates a variable x and gives it the value 5. Then it creates another variable y and sets it to the same value as x
    // This is because integers are simple data types.
    let x = 5;
    let y = x;

    /* 
     * However, this just creates another string s2 and sets its contests to the same as s1. That is: 
     *  
     *  s1:
     *  FIELD |       ptr       | len | capactity 
     *  VALUE | ptr to contents |  5  |     5
     * 
     *  So, s2 will just have the same contents as s1 has (shown above). It will have the same value because it will be
     *  pointing to the same direction as s1. 
     *
     *  Furthermore, to avoid double free errors when s1 and s2 go out of scope (i.e. inner calling of the drop() function), 
     *  Rust considers s1 as an invalid variable, so you won't be able to use it anymore.
     * 
     *  This is know in Rust as moving a variable (move), and allows us to only free one variable.
     * 
    */ 
    let s1 = String::from("hello");
    let s2 = s1;

    // To avoid this, we can create a deep copy with the clone method. Note with the use of it, two variables have to be freed.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    /* 
     * Data types such as Integers don't need the clone method to have their value copied, as their size is known in compile time 
     * and are stored on the stack. This data types are:
     *  
     *  All integer types (u32, i16, etc)
     *  The boolean type
     *  All floating point types
     *  The char type
     *  Tuples when they contain types wich implement copy(), such as (i32, i32); but not (i32, String)
     *  
    */

    // OWNERSHIP AND FUNCTIONS
    // s comes into scope
    let s = String::from("hello");
    
    // s value moves into function and is no longer valid in main.
    takes_ownership(s);

    // x comes into scope
    let x = 5;

    // x would move into the function, but as i32 implements Copy, it is okay to use it after this point.
    makes_copy(x);


}

// When exiting from this function, some_string (aka s) goes out of scope and so drop() is called. Heap memory is freed.
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// As i32 is stored in the stack, some integer just comes in and out of scope with nothing special happening.
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
