/*
    you can think of if let as syntax sugar
    for a match that runs code when the value
    matches one pattern and then ignores all other values.
*/

fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}