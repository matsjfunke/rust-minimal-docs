fn main() {
    if_else();
    loop_counter(3);
    conditional_loop(2);
    iterate_takeoff();
}

fn if_else() {
    let condition = false;
    let number = if condition { 5 } else { 6 }; // if is an expression, we can use it on the right side of a let statement to assign the outcome to variable

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loop_counter(starting_number: i32) {
    let mut counter = starting_number;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn conditional_loop(counter: i32) {
    let mut counter: i32 = counter;
    while counter < 10 {
        println!("{counter}");
        counter += 1;
    }
    println!("happy new year");
}

fn iterate_takeoff() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("takeoff")
}
