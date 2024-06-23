use std::io;

fn main() {
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
                              "August", "September", "October", "November", "December"];

    loop {
        println!("Enter a number (between 0-11) to get the corresponding month:");

        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: Result<usize, _> = index.trim().parse();

        match index {
            Ok(num) if num < 12 => {
                println!("The value of the element at index {num} is: {}", months[num]);
                break;
            },
            Ok(_) => println!("Number is out of range. Please enter a number between 0 and 11."),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
