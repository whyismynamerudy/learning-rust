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


*/