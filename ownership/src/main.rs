fn main() {
    string_on_heap();
    // string_borrow_error()
    string_borrow_clone();


}

fn string_borrow_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // copies pointer to string refered to by s1

    println!("s1 = {}, s2 = {}", s1, s2);
}

// fn string_borrow_error() {
//     let s1 = String::from("hello");
//     let s2 = s1; // copies pointer to string refered to by s1

//     println!("{}, world!", s1);
// }

fn string_on_heap() {
    { // s isn't yet declared, so it isn't valid
    let mut s = String::from("Hello"); // String::from constructor allocates memory on heap
    s.push_str(", world!"); // push_str() appends a literal to the string
    println!("{}", s); // print 

    } 
    // Scope is over, memory for s is returned using drop(s)
    // This is similar to the Resource Acquisition is Initialization (RAII) pattern in C++
}
