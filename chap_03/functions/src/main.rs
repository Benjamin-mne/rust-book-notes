/*
    Statements are instructions that perform some action and do not return a value.
    Expressions evaluate to a resultant value. Let’s look at some examples.

        let y = 6; is a statement.
        Function definitions are also statements.
        5 + 6, which is an expression that evaluates to the value 11
        Expressions can be part of statements
 */

fn main() {
    /*
        Expressions do not include ending semicolons. 
        If you add a semicolon to the end of an expression, 
        you turn it into a statement, and it will then not return a value. 
    */
    let y = {
        let x = 3;
        x + 1
    };


    println!("The value of y is: {y}");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let one_more = plus_one(10);
    println!("The value of one_more is: {one_more}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}