use std::io;

fn main() {
    let mut unit = String::new();
    loop {
        println!("Do you want to convert to Fahrenheit or Celcius? Type F or C.");

        io::stdin().read_line(&mut unit).expect("Failed to read line");

        unit = unit.trim().to_uppercase();
        println!("unit is: {unit}");
        if unit == "F" || unit.trim() == "C" {
            break;
        } else {
            println!("Please type F or C");
            unit = String::new();
        }
    }
    
    loop {
        let other_unit: String;
        if unit == "F" {
            other_unit = "C".to_string();
        } else {
            other_unit = "F".to_string();
        }

        println!("Please enter a temperature in {other_unit} to convert to degrees {unit}.");
        
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        let converted_temp: f32;
        if unit == "F" {
            converted_temp = temp * 9.0 / 5.0 + 32.0;
        } else {
            converted_temp = (temp - 32.0) * 5.0 /9.0;
        }

        println!("The converted temp is {:.1} {}", converted_temp, unit);
        break;
    }
}
