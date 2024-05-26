fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    let pi: f32 = 3.14;
    println!("Value of pi: {}", pi);


    let is_true: bool = true;
    let is_false: bool = false;

    println!("Is true: {}", is_true);
    println!("Is false: {}", is_false);

    let letter: char = 'a';
    println!("The first letter of the alphabet: {}", letter);
}