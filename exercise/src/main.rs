// fn main(){
//     println!("Hello, world!");
//     println!("I'm Rudy and I'm 21 and I love hip-hop.");
// }

fn main() {
    let x: i32 = 21; // this in immutable; variables in rust are immutable by default
    println!("my age is {} and via function is {}.", x, my_age_is());

    let arr: [i32; 5] = [0, 1, 2, 3, 4];

    println!("arr is {arr:?}");

    let slice: &[i32] = &arr[1..4];

    println!("slice s is {slice:?}");

    // arr[3] = 6;

    // println!("slice s after mutating arr is {slice:?}");

    // this would not work:
    // x = 22 ( 😔 )
    // println!("{}",fibbi(3));
    // let arr = fibbo(5);
    // println!("{arr:?}");
}

// Implementation below is wrong/
// fn fibbo(n: i32) -> [i32; n] {
//     let mut arr: [i32; n] = [-1; n];
//     for element in 0..n {
//         let temp = fibbi(element, arr);
//         arr[element] = temp;
//     }

//     arr
// }

// fn fibbi(n: i32, arr: [i32; n]) -> i32 {
//     if arr[n] != -1 {
//         return arr[n];
//     }

//     if n<2 {
//         arr[n] = n;
//         return n;
//     } else {
//         let temp = fibbi(n-1, arr) + fibbi(n-2, arr);
//         arr[n] = temp;
//         return temp;
//     }
// }

// example function, will learn more a bit later:
// fn interproduct(a: i32, b: i32, c: i32) -> i32 {
//     return a * b + b * c + c * a;
// }

fn my_age_is() -> i32 {
    21
}