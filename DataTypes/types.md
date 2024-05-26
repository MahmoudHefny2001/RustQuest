```
---- Primitive data types in Rust
---- int, float, bool, char
```

### Integers

```
(rust has signed [+ and - ] and unsigned integer [only +] types of different sizes)


. i8, i16, i32, i64, i128: signed integers.

. u8, u16, u32, u64, u128: unsigned integers.

. i32 stores up to 32 bits of data and i64 stores up to 64 bits of data.
. range: 
    i32: from -2147483648 to 2147483647
    i64: from -9223372036854775808 to 9223372036854775807

```

```rust
fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
}
```

### Floats [Floating point types]

```
f32, f64
```

``` rust

let pi: f32 = 3.14;
println!("Value of pi: {}", pi);

```

### Boolean: true, false

``` rust
let is_true: bool = true;
let is_false: bool = false;

println!("Is true: {}", is_true);
println!("Is false: {}", is_false);

```


### Charachter: Type - char

``` rust

let letter: char = 'a';

println!("The first letter of the alphabet: {}", letter);

```