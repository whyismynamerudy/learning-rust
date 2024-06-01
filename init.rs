fn main() {
    println!("Hello, world!");
}

// typing rustc init.rs builds the file into an executable
// to run, type ./init

// when using cargo, follow these steps:
// 1. cargo new <folder-name> to build a new cargo package in directory. 
// 2. cargo run to build and run your code

// cargo is preferred in the rust community as it gives you the standard project structure
    // similar to npm init ig

/*
Use cargo check to quickly check your project for errors, use cargo build to compile it without running it.
You will find the output in target/debug/ for a normal debug build. Use cargo build --release to produce an 
optimized release build in target/release/.

Add dependencies for your project by editing Cargo.toml. When you run cargo commands, it will automatically 
download and compile missing dependencies for you.

The types have widths as follows:
    . iN, uN, and fN are N bits wide,
    . isize and usize are the width of a pointer,
    . char is 32 bits wide,
    . bool is 8 bits wide.

Rust has type inference - it determines type based on use and init value.
When nothing constrains the type of an integer literal, Rust defaults to i32. 
This sometimes appears as {integer} in error messages. Similarly, floating-point literals default to f64.

if statements:
you can use if as an expression. last expression of each block becomes the value of the if expression:
let size = if x < 20 { "small" } else { "large" };

for loop:
can interate over ranges or elements.
for x in 1..5 (1, 2, 3, 4) AND
for elem in [1, 2, 3, 4, 5]

loop:
loop statements iterate until a break (akin to while True ig)
let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }

Both continue and break can optionally take a label argument which is used to break out of nested loops:
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {        // note the 'outer label here, used by break to break out here
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");

Blocks:
contains a sequence of expressions enclosed by braces {}.
each block has its own value and type, which is that of the last expression in the block.
    . hence, if the last expression ends with ;, then the resulting value and type is ().
eg:
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y       // note the lack of ; in the last expression
    };
    println!("x: {x}");
note that the scope of the block is limited to its enclosing block, so you can shadow variables from outside.

Functions:
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}
Some functions have no return value, and return the ‘unit type’, (). The compiler will infer this if the -> () return type is omitted.

Useful Macros:
dbg!(...) -> logs the value of the expression and returns it

Arrays:
A value of the array type [T; N] holds N (a compile-time constant) elements of the same type T. 
Note that the length of the array is part of its type, which means that [u8; 3] and [u8; 4] are 
considered two different types.
The println! macro asks for the debug implementation with the ? format parameter: {} gives the 
default output, {:?} gives the debug output. Types such as integers and strings implement 
the default output, but arrays only implement the debug output. This means that we must use 
debug output here.
Adding #, eg {a:#?}, invokes a “pretty printing” format, which can be easier to read.
fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
}

Tuples:
Tuples group together values of different types into a compound type.
Fields of a tuple can be accessed by the period and the index of the value, e.g. t.0, t.1.
The empty tuple () is referred to as the “unit type” and signifies absence of a return value, 
akin to void in other languages.
fn main() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}
*/