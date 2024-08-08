fn main() {
    // if_expressions();
    repetition_with_loops();
}

fn if_expressions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // Using if in a let Statement
    let condition = true;
    let other_number = if condition { 5 } else { 6 };

    println!("The value of other_number is: {other_number}");
}

fn repetition_with_loops() {
    // loop
    
    let mut cooldown = 10;

    let state = loop {
        println!("Wait: {cooldown}");
        cooldown -= 1;

        if cooldown == 0 {
            break true;
        }
    };

    println!("Allowed is {state}");

    // while loop

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

}