fn main() {
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // String slices
    let s = String::from("Hello word");
    let word = first_word(&s);

    println!("The first word is: {word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // Convert our String to an array of bytes using the as_bytes method
    // Next, we create an iterator over the array of bytes using the iter method
    // iter is a method that returns each element in a collection 
    // and that enumerate wraps the result of iter 
    // and returns each element as part of a tuple instead.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // String Slice
        }
    }

    &s[..]
}