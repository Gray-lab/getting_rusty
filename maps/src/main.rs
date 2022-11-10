// HashMaps must be imported from the standard library
use std::collections::HashMap;

fn main() {
    // No macro is available for consructing HashMaps
    let mut scores = HashMap::new();

    // HashMaps must be homogenous
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Values can be accessed with get()
    let team_name = String::from("Blue");

    // get returns an Option<&V>, which we copy to get an Option<i32> instead of Option<&i32>
    // Option is then unwrapped to get either the score or 0 if there is no entry
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team {}'s score is {}", team_name, score);

    // We can also iterate over the values in a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // types that implement Copy such as i32 are copied into the hash map
    // owned values like String are moved and the hash map becomes the owner
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    // field_name and field_value are no longer valid
    // println!("{}", field_name);  <- this causes an error

    // We could insert references to the values instead, but we need to validate their lifelines. More in Ch 10.

    // Hash maps can be update by either overwriting, not overwriting, or modifying the values
    // Replacing an existing value:
    map.insert(String::from("Favorite color"), String::from("Green"));
    println!("{:?}", map);

    // Adding a key and value only if key isn't already present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // entry(key) returns an enum called Entry which represents a value that might or might not exist
    // or_insert(value) inserts a value if the Entry enum is vacant
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // either create the entry with 0, or bind mutable reference to entry to count
        let count = map.entry(word).or_insert(0);
        // Dereference the pointer so we can increment the value in the hash map
        *count += 1;
    }

    println!("{:?}", map);

}
