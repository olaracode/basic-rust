# More about typing

## Integers

In rust we have different ways to write integers

```rs
let a: i32 = 98_222; // Decimal
let b: i32 = 0xff; // Hexadecimal
let c: i32 = 0o77; // Octal
let d: i32 = 0b1111_0000; // Binary
let e: u8 = b'A'; // Byte (u8 only);
```

All of the different integer types have max values that can be assigned

> [Integer Data Types](./readme.md/#Data_types)

```rs
let f: u8 = 255; // Max value of u8
```

## Tuples

Tuples are related values stored in a comma separated parenthesis with a fixed length

```rs
fn main(){
    let tup: (&str, i32) = ("Rusteeze", 100_000);
    // To get values from a tuple we have two options
    // Destructuring and dot notation
    let (channel: &str, sub_count: i32) = tup;
    let sub_count = tup.0;
}
```

## Arrays

Like tuples arrays are a list of related values in a comma separated `square brakets`. They are declared with the data type that it will hold and the fixed length

```rs
fn main(){
    // In this array rust will infer the type and lenght
    // Giving it a type of [i32, _] where the underscore
    // is the length of the array
    let error_codes = [200, 404, 500];

    // Here instead we use declarative typing
    // And the [value; count] notation to create an
    // array with n
    let byte: [i32, 8] = [0; 8];
}
```
