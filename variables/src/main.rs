fn main() {
    const THIS_IS_A_THREE: u32 = 3;
    // let mut x = 5;
    // println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x in the outer scope is: {x}");
    {
        // An immutable variable can still be shadowed
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x in the outer scope is still: {x}");
    println!("The value of the constant is: {THIS_IS_A_THREE}");

    let spaces = "    ";
    println!("The string spaces: {spaces}");
    // This works even though spaces isn't mutable because we are creating a 
    // new variable by using let. This even lets us change the type.
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");

    let x_float = 2.0; // f64 by default
    let y_float: f32 = 3.0; // f32 by explicit typing

    println!("x_float: {x_float}, y_float: {y_float}");
    
    // operations
    let _sum = 5 + 10;

    let difference = 95.5 - 4.3;
    let product = 4 * 30;

    let _quotient = 56.7 / 32.2; // floating point division
    let _floored =  2 / 3; // integer division, result is 0

    let _remainder = 43 % 5; //

    // booleans
    let _t = true; // implicit boolean
    let _f: bool = false; 

    // char type - represents a 4 byte Unicode Scalar Value. 
    // Ranges from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    let _c = 'z'; // char literals specified with single quotes
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';

    // compound types: tuples and arrays
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // type annotations are optional

    // tup binds to the entire tuple. Use patern matching to destructure:
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z:{z}");

    // arrays - kind of feel like arrays in c, with directly allocated memory
    let _a = [1, 2, 3, 4, 5]; // allocated on heap, fixed length
    let _b: [i32; 5] = [1, 2, 3, 4, 5];  // explicitly type annotated
    let _c = [3; 5]; // initializes an array with 5 elements all set to 3: [3, 3, 3, 3, 3]

    // Rust will not allow out of bounds array access. 
    // If not able to be caught at compile time it will panic at runtime

}
