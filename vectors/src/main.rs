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

}
