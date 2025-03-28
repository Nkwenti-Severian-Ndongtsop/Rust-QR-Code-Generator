use std::io;



pub fn user_input() -> (String, Option<u32>, String) {
    let mut input1 = String::new();

    println!("Enter the data to transformed (url/text)");

    let data = match io::stdin().read_line(&mut input1) {
        Ok(_) => {
            let binding = input1.trim().to_string();
            binding
            
        },
        Err(e) => {
            eprint!("Couldn.t collect input: {}", e);
            return (String::new(),  None, String::new());
        }
    };

    let mut input2 = String::new();

    println!("Enter the QR-code size (default[300])");

    let size = match io::stdin().read_line(&mut input2) {
        Ok(_) => match input2.to_string().trim().parse::<u32>() {
            Ok(value) => {
                value
            },
            Err(e) => {
                eprintln!("Error parsing value: {}. Using default size 300.", e);
                300
            },
        }
        Err(e) => {
            eprint!("Couldn.t collect input: {}", e);
            return (String::new(), None, String::new());
        }
    };

    let mut input3 = String::new();
    println!("Enter the format output you want (jpeg, bmp, png, webp)");

    let format = match io::stdin().read_line(&mut input3) {
        Ok(_) => {
            let trimmed = input3.trim().to_string();
            trimmed
        },
        Err(e) => {
            eprint!("Couldn.t collect input: {}", e);
            return (String::new(), None, String::new());
        }
    };

    (format.to_string(), Some(size), data.to_string())
}
