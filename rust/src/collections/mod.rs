use std::collections::HashMap;

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    /* VECTORS */

    /* A Rust vector is like a List in other languages. You can either create an empty vector or
     * instantiate it with some values using the vec! macro. Here are two examples, one for each
     * situation. Note that if you don't add values at declaration, Rust won't be able to infer
     * the type of the values stored within and you will have to state it.
     */
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    /* If you wan't to add values to a vector, you must declare it with let mut so that it is a
     * mutable variable. Then you can push values into it using dt notation and the push method.
     * Rust will infer the type from the fisrt value that you push into the vector.
     */
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    /* To get a value from a vector, you can either use the get method or indexing. The following
     * examples show a way of doing it with both methods. Note that we use an Option<&i32> type when
     * using the get method, because this way we can check wheter or not the desired position exists
     * by using a match statement.
     */
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(thrid) => println!("The third element is {}", thrid),
        None => println!("There is no third element"),
    }

    /* Note that when using vectors, if you associate an element of a mutable vector to an inmutable
     * variable, the program will crash due to the ownership and borrowing rules and is like this to
     * avoid memory problems when handling vectors. With that in mind, you now know the following code
     * snipped is incorrect and will not compile:
     *          let mut v = vec![1, 2, 3, 4, 5];
     *          let first = &v[0];
     *          v.push(6);
     * In this example, there is a mutable borrow when calling the push method. That means you are using
     * the 0th element in both a mutable and inmutable enviroment.
     */

    /* Example of iteration over the elements in a vector */
    let v = vec![100, 32, 27];
    for i in &v {
        println!("{}", i);
    }

    /* If you want to modify the element inside a vector while iterating over them, you will have to use
     * the * dereference operator to get to the value in i before using the += operator
     */
    let mut v = vec![100, 32, 57];
    for i in &v {
        *i += 50;
    }

    /* In some cases, you may want to store different data types inside a vector. This can be done by using
     * an enum type that holds the different data types you want to store. You will have to make use of match
     * statements depending on the instance of the enum stored to avoid memory issues.
     */
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue)")),
        SpreadSheetCell::Float(10.12),
    ];

    /* As any other types, vectors are freed when it goes out of scope, dropping also all of its elements. */
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    /* STRINGS */

    /* A String can be created in one of the following ways. These are using the new method (same as vector's new
     * method), creating it from a string literal (&str), using the to_string method (implemented by all types
     * that implement the Display trait)
     */
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    /* As Strings are UTF-8 encoded, all of the following are valid values for a String:  */
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    /* To append to a String we can use several methods. we will explore them one at a time, starting with the use of
     * the push_str and push methods. We can use them to append a string slice wich we are not interested in keeping
     * after it insertion, or also use is for mantaining ownership of a value.
     */
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    /* Differently from the push_str method, which appends a whole string slice to a String, the push method adds a
     * single character to it
     */
    let mut s = String::from("lo");
    s.push('s');

    /* We can also use the '+' operator to combine Strings. This operator makes a call to the add method, which looks like
     * this:
     *          fn add(self, s: &str) -> String
     * aving a look at this, we can assume that the first String of the expression will no longer be valid, but the
     * second one will, as it's passed as a reference to the second String. Also, we can see another ability of the
     * Rust compiler. It can coerce a &String into a &str, wich in the example will turn &s2 into &s2[..]. In
     * conclusion, the Rust's '+' operator doesn't make a bunch of copies, it instead uses clever use of Strings to
     * be much more efficient
     */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    /* As ypu can see in the example just above this comment, using the '+' operator can quickly get messy. Thats when the
     * format! macro comes in. This macro works similarly to the printtl! one, but it returns a String and uses references,
     * so it doesn't take ownership of any of its parameters.
     */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    /* Rust does not allow String indexing to avoid problems. This is because Strings are, in fact, of type Vector<u8>. It
     * would be problematic with alphabets other than the latin one to allow indexing. It will also take more time than the
     * allowed for a get operation (will take > O(1)). Rather than indexing, Rust allows String slicing. Some considerations
     * must be taken, as you can see in the examples below.
     */
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // s will be Зд instead of Здра, as each of the values take up 2 bytes.
    let s = &hello[0..1]; // will trigger panic at runtime, as you can't slice part of a character's bytes.

    /* You can iterate over a String by specifying if you want to iterate over chars or bytes. The code snippets below show both
     * possibilities.
     */
    for c in "Зд".chars() {
        println!("{}", c);
    }

    /* Output will be:
     * З
     * д
     */

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    /* Output will be:
     * 208
     * 151
     * 208
     * 180
     */

    /* HASHMAPS */

    /* To use HashMaps, you will have to use the Hashap from std::collections::HashhMap. You can create an empty HasMap and insert
     * key/value pairs after instantiation. Note that there are not any marcos fro this data structure.
     */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /* To access to the values stored in a HashMap, you use the get method, and tell it the key you want the value of. In the example below,
     * get() returns an Option<&V>, copied() will return an Option<V> and unwrap_or(0) will unwrap V from the Option and give its value to
     * score. If Option<&V> is None, this last method will give score the value of 0.
     */
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    /* You can also iterate over a HashMap's keys values in a similar kay as you can with a vector: */
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /* HashMaps don't take ownership of the values introduced into them if they are of types which implement the Copy trait. For owned values
     * like Strings, you can insert them as references to the values, but these references must be valid at least for as long as the HashMap is.
     * 
     * There are some ways to update a HashMap. We will discuss three of them.
     */
}
