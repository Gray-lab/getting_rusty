fn main() {
    let s = String::new(); // A string is implemented as a wrapper
                               // around a vector of bytes
    println!("{}", s);  

    // a literal (or any type that implements Display trait) can be turned into a string with to_string()
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);  

    // the method also works directly on a literal
    let s = "initial contents".to_string();
    println!("{}", s);  

    // or we can use String::from
    let s = String::from("initial contents");
    println!("{}", s);  

    
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s); 

    // push_str() takes a string slice so that it doesn't take ownership of the parameter
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push() appends one character to the end
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // + can be used to concatenate strings, but has some subtleties associated with it
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is moved to s3 and s2 needs to be type &str. 
                               // &s2 (&String) is deref coerced into &str[..]
    println!("{}", s3);
    // s2 is still a valid String after the operation, but s1 has been moved
    println!("{}", s2);

    // Concatenation works for complex strings, but format! macro is sometimes more legible
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // Instead using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    // format! only use references so it does not take ownership of any parameters
    println!("{},{},{}->{}", s1, s2, s3, s);

    // Rust Strings cannot be indexed into:
    // Doesn't compile:
    // let s1 = String::from("hello");
    // let h = s1[0];
    // Slices can be used to select specific bytes
    let hello = "こんにちは".to_string();
    let s = &hello[0..6];
    println!("{}[0..6] = {}", hello, s);

    // To iterate over strings we can either use .bytes or .chars
    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{:#04x}", b);
    }

    // libraries exist to allow getting grapheme clusters from strings (crates.io)


}
