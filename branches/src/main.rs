fn main() {
    let num = 7;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // value after break is returned out of the loop
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    // loops can be labeled
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break statements can then terminate outer loops
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value of the element is: {element}");
    }
}
