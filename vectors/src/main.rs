fn main() {
    // New empty vector for i32 (no values, so type must be provided)
    // Vectors are implemented using a generic: Vec<T>
    let v: Vec<i32> = Vec::new();

    // New vector initialized with values using vec! macro
    let v2 = vec![1, 2, 3];

    // Since we place items in the vector, Rust can infer the type
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("{:?}", v3);

    let v4 = vec![1, 2, 3, 4, 5];

    let fourth: &i32 = &v4[3]; // Access fourth element of v4 - this will panic if element doesn't exist

    let third: Option<&i32> = v4.get(2); // This will return an Option::None if element doesn't exist
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v5 = vec![100, 32, 45];
    for i in &v5 {
        println!("{}", i);
    }

    let mut v6 = vec![100, 32, 45];
    for i in &mut v6 {
        *i += 50;
        println!("{}", i);
    }
}
