// fn main(){
//     println!("Hello, world!");
//     println!("I'm Rudy and I'm 21 and I love hip-hop.");
// }

fn main() {
    let x: i32 = 21; // this in immutable; variables in rust are immutable by default
    println!("my age is {} and via function is {}.", x, my_age_is());

    // this would not work:
    // x = 22 ( 😔 )
}

// example function, will learn more a bit later:
// fn interproduct(a: i32, b: i32, c: i32) -> i32 {
//     return a * b + b * c + c * a;
// }

fn my_age_is() -> i32 {
    21
}