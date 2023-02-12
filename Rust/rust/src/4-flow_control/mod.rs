fn main() {
    let number = 3;

    if number < 5 {
        println!("true");
    } else if number > 5 {
        println!("false");
    } else {
        println!("equal");
    }

    let condition = true;
    
    // Types can't be different, underscorde is used to tell rust this value is unused
    let _number = if condition { 5 } else { 6 };

    // infinite loop
    // loop { }

    // break and continue keywords work same as in python

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

    // We use the semicolon here because is the end of an asignation
    };

    println!("The result is {result}");

    // Use of tags to identify nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;

    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is {element}");
    }

    // For loop through the items 1 to 4 in rev (reverse) order
    for number in (1..4).rev() {
        println!("{number}!");
    }

}
