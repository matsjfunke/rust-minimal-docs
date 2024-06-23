# Installation / Update
```bash
# install with rustup:
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# update_
rustup update
```

# Cargo: system and package manager
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory
```bash
# create a project
cargo new <name>
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

###Integers
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
 
###Floats
- all signed
- f32 & f64 
- default type f64 because on modern CPUs roughly same speed as f32 but more precise.

###Booleans
- size = 1 bite
```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```
###Characters
char literals with single quotes, type is four bytes in size and represents a Unicode Scalar Value, meaning it can represent a lot more than just ASCII

###Strings
- literals which use double quotes.
- all UTF-8
1. String -> create or modify strings
2. &str (string slice) -> read/analyze strings

```rust
let x: char = 'hello';
let x: String = "hello";
```

##Compound Types
###Tuple ()
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

###Array []
- elements of array must have same type.
- arrays in Rust have a fixed length.
```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];
```

#Functions
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
