fn main() {
    let mut s1 = String::new();
    println!("Enter a string: ");
    std::io::stdin().read_line(&mut s1).unwrap();
    trim_newline(&mut s1);
    
    let len = get_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}