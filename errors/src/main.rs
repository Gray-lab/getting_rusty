use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    // panic!("crash and burn!");

    // let v = vec![1, 2, 3];

    // v[99];

    // Opening a file yields a Result value
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        // matching on the Result
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // File::create also returns a Result value that we can match on
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
        },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // we can use methods on Result to simplify this logic
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // we can also use unwrap or expect to get a panic if the result is an error
    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in the project");
}

// functions can also propagate an error to the calling fuction
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// this can be simplified by using the ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// we could chain the calls to shorten this even further
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// std::fs provides a read_to_string function that returns the Result<String, io:Error>, taking care of all the error handling
use std::fs;

fn read_username_from_file4 () -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}