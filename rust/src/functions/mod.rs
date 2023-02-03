fn main() {
    print_labeled_measurement(5, 'h');
    // Print the result of a function
    println!("{}", five());                                             
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // No semicolon because it is an expression
    5                                                                   
}
