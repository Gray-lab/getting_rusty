fn main() {
    let s = String::from("This is a test");
    println!("{}", first_word(&s));
    println!("{}", second_word(&s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut first_index: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        let mut count: usize = 0;
        
        if item == b' ' {
            count += 1;
            if count == 1 {
                first_index = i;
            }
            if count == 2 {
                return &s[first_index+1..i];
            }
        }
    }
    &s[first_index+1..]
}