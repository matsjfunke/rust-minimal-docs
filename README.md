# Table of Contents
1. [Installation / Update](#installation--update)
2. [Compile / run](#compile--run)
3. [Cargo: system and package manager](#cargo-system-and-package-manager)
4. [Variables](#variables)
5. [Types](#types)
   - [Integers](#integers)
   - [Floats](#floats)
   - [Booleans](#booleans)
   - [Characters](#characters)
   - [Strings](#strings)
6. [Compound Types](#compound-types)
   - [Tuple ()](#tuple-)
   - [Array []](#array-)
7. [Functions](#functions)
8. [Control Structure](#control-structure)
   - [if, else](#if-else)
   - [loops](#loops)
9. [Ownership (managing computer memory)](#ownership-managing-computer-memory)
   - [Stack & Heap](#stack--heap)
   - [Ownership rules](#ownership-rules)
   - [Variable Scope](#variable-scope)
   - [References & Borrowing](#references)
10. [Structs, structuring related data](#structs)
   - [Tuple Structs](#tuple-structs-w/o-names)
   - [Methods](#struct-methods-(functions-inside-structs)

# Installation / Update
```bash
# install with rustup:
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# update_
rustup update
```

# Compile / run
```bash
# create rust file
echo 'fn main() { println!("Hello, World!"); }' > hello.rs
# compile
rustc main.rs
# run
./main
```

# Cargo: system and package manager
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory
```bash
# create a project
cargo new <project-name>
# build a project
cargo build
# build and run a project in one step
cargo run
# build a project without producing a binary to check for errors
cargo check
# update dependencies
cargo update
```

# Variables
- all immutable by default
```rust
let x = 5; // immutable
let mut x = 5; // mutable

const x = 5; // always immutable & typed
```

**shadowing** (reusing name)
- must be same type
- only works in same scope
```rust
let x = 5;
let x = x + 5;
println!({x}); // 10
```

# Types

### Integers
Signed and unsigned refer negative or positive 
- whether the number needs a sign (signed) or it will only ever be positive and can therefore be represented without a sign (unsigned).
- default = i32

length | signed    | unsigned
-------|-----------|------------
8-bit  |     i8    |     u8
16-bit |     i16   |     u16
32-bit |     i32   |     u32
64-bit |     i64   |     u64
128-bit|     i128  |     u128
arch   |     isize |     usize
 
### Floats
- all signed
- f32 & f64 
- default type f64 because on modern CPUs roughly same speed as f32 but more precise.

### Booleans
- size = 1 bite
```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```
### Characters
char literals with single quotes, type is four bytes in size and represents a Unicode Scalar Value, meaning it can represent a lot more than just ASCII

### Strings
- literals which use double quotes.
- all UTF-8
1. String -> create or modify strings
2. &str (string slice) -> read only (immutable) [more on references later](#references)


```rust
let x: char = 'hello';
let x: &str = "hello";
```

## Compound Types
### Tuple ()
- grouping a variety of types
- fixed length: once declared, they cannot grow or shrink in size.
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {y}"); // 6.4

    let five_hundert = tup.0
    println!("value of index 0 is: {five_hundert}"); // 500
}
```

### Array []
- elements of array must have same type.
- arrays in Rust have a fixed length.
```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];
```

# Functions
```rust
 // Statements = intructions for  actions that dont return a value.
let y = 6;
// Expression evaluate to a resultant value.
let y = {
    let x = 3;
    x + 1
};
```
- main function / entrypoint at top of file
- returning values
```rust
fn main() {
    let result = sum(5, 10);
    println!("The sum is: {result}");
}
fn sum(a: i32, b: i32) -> i32 { //don’t need to name return values, but we must declare their type after arrow ->
    a + b // implicit return
    // or
    return a + b // explicit return

}
```

# Control Structure
## if, else
```rust
fn main() {
    let condition = true;
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
```
## loops
```rust
// loop -> used to loop infintely until break
loop {
     break;
}
// while / conditional loop -> until false
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
// for loop -> used for iterating
fn main() {
    for number in (1..4).rev() { // rev method, to reverse the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

# Ownership (managing computer memory)
- python for example has "garbage collection" that regularly looks for no-longer-used memory as the program runs.
- in other languages, the programmer must explicitly allocate and free the memory.
- rust manages memory through a system of ownership with a set of rules that the compiler checks.

## Stack & Heap
**Stack** stores values in the order it gets them and removes the values in the opposite order -> **last in, first out**
- think stack of books, new book layed on top is the first to get picked but.

**Heap** less organized: putting data on heap, requests certain amount of space, memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location of the data
- think of a table where you can place objects anywhere there's space. To find an object later, you need to remember its exact location on the table.
```rust
let x: i32 = 10; // Allocated on the stack
let s = String::from("hello"); // Allocated on the heap
```

## Ownership rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### copying variables
```rust
// stack
let x = 5;
let y = x;
println!("x = {x}, y = {y}");

// heap
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {s1}, s2 = {s2}");
```

## Variable Scope
- variables are only accessable if the parrent is in scope
```rust
{                      // s is not valid here, it’s not yet declared

    let s = "hello";   // s is valid from this point forward
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

## References
**Ownership problem:** ownership is transfer
- when a function takes ownership of a value, the original variable can no longer be used unless the ownership is returned -> cumbersome and unnecessary

**Solution: using References**
- References allow you to refer to a value without taking ownership
- use "&" to create references that borrow data without taking ownership.

**Borrowing** -> accessing a variable's value through a reference
```rust
// reference example
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // & references s1

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // calculate_length borrows reference to s
    s.len()
}

// mutable reference example
fn main() {
    let mut s = String::from("hello"); // mut makes s mutable

    change(&mut s); // &mut references to s and shows mutablility
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```


# Structs
similar to [tuple](#tuple-): pieces of struct can be different types but in struct each piece of data has a name to clarify purpose
```rust
// define a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// to use struct create an instance of that struct by specifying values
fn main() {
    let mut user1 = User { // entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // access specific value with dot-notation

    // Creating Instances from Other Instances with Struct Update Syntax
    let user2 = User {
        active: user1.active, 
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

// Function that returns a User instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // type is already defined in parameter
        email,
        sign_in_count: 1,
    }
}
```

## Tuple Structs w/o names
```rust
struct Color(i32, i32, i32);
struct Cursor(i32, i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let current_location = Cursor(0, 0, 0);
}
```

## Struct Methods (functions inside structs)
- impl stands for implementation aka. rust method
```rust
#[derive(Debug)] // Debug trait enables to print struct in a way we can see its value while we’re debugging 
struct Rectangle {
    width: u32,
    height: u32,
}

// method definition
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
