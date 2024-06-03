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
let (left, right) = tuple;

Shared References:
"borrowing", same as pointers in C. 
Shared references are read-only, and the referenced data cannot change.
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
above, r cannot alter the value of a/b

Exclusive References:
Mutable references, allow changing the value they refer to.
“Exclusive” means that only this reference can be used to access the value. No other references 
(shared or exclusive) can exist at the same time, and the referenced value cannot be accessed while
the exclusive reference exists.
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
Can't have any other type of refernce to point.0 while there is an &mut ref.

Slices:
Gives you a view into a larger collection.
fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}
Notice that the type of s (&[i32]) no longer mentions the array length. This allows us to perform 
computation on slices of different sizes.
Note that you cannot alter a once the slice has been created.

Strings:
There are two string types in Rust:
    . &str is a slice of UTF-8 encoded bytes, similar to &[u8].
    . String is an owned buffer of UTF-8 encoded bytes, similar to Vec<T>.
fn main() {
    let s1: &str = "World";  // this is an imutable reference to string (string literal)
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello "); // owned, creates a string from string literal
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");
}

ON TO STRUCTS!!

Named Structs:
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person { name: String::from("Peter"), age: 27 };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person { name: String::from("Jackie"), ..avery };
    describe(&jackie);
}
The syntax ..avery allows us to copy the majority of the fields from the old struct without having 
to explicitly type it all out. It must always be the last element.

Tuple Structs:
Use when field names are unimportant.
struct Point(i32, i32);
Often used for single-field wrappers:
struct PoundsOfForce(f64);
struct Newtons(f64);

Enums:
'enum; keyword allows for the creation of a type with a few variants:
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);
}
Enumerations allow you to collect a set of values under one type.

Static:
Static variables will live during the whole execution of the program.
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}

Const:
Constants are evaluated at compile time and their values are inlined wherever they are used:
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}

Const vs Static:
A constant in Rust is immutable. You neither can reassign nor modify it.
A static variable can be mutable and therefore can either be modified or reassigned. Note that 
writing/modifying a global static variable is unsafe and therefore needs an unsafe block.
When you compile a binary, all const "occurrences" (where you use that const in your source code) will be replaced by that value directly.
statics will have a dedicated section in your binary where they will be placed.
In conclusion, 
const:
    . Have no fixed address in memory
    . They’re inlined to each place which uses them, this means they are put directly into the binary on the places which use them.
    . Usually faster runtime but bigger executable file because it doesn't have to look up an address like static
static:
    . Have a fixed address in memory
    . Their value is loaded from this fixed address each place which uses them.
    . Usually slower runtime because we need to perform the extra instruction of loading the data from the fixed address. However this 
      could result in a smaller executable file (only when it is used frequently) because it doesn't have to have multiple copies of 
      the value baked into the executable.

Type Aliases:
enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;
*/