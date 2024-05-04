# Rust introduction

## Creating a new rust environment

To create a new working directory for rust you can use cargo

```sh
cargo new working_directory_name

# example
cargo new example
# enter the working directory using cd
cd exercise
```

Then compile your code and run it you can use `cargo run`

```sh
# /exercise
cargo run
# Compiling exercise v0.1.0 (/home/mgeisler/tmp/exercise)
#     Finished dev [unoptimized + debuginfo] target(s) in 0.75s
#      Running `target/debug/exercise`
# Hello World!
```

To simply compile your code without executing it you can use `cargo build`

To add dependencies to your rust project you can use Cargo.toml and the next time you use a cargo command the missing dependencies will be downloaded and compiled

## What is Rust

Rust is a new language that launched their 1.0 version on 2015.

Rust is a compiled language similar to C++

Rust can be used on a gran variety of devises such as:

- Firmware and initial loaders
- Smart Screens
- Smart phones
- Desktop computers
- Servers

Rust satisfies the same needs as C++:

- Great flexibility
- High level of control
- Can be vertically reduced to very limited devises such as microcontrolers
- Doesn't have a runtime nor garbage collection
- It is centered on realiability and safety without taking from the performance

## Hello world

Lets talk about the simples rust program, a classic Hello World:

```rs
fn main(){
	println!("Hello World");
}
```

What we can see here:

- Functions are introduces with the `fn` keyword
- The blocks of code are marked with _curly brackets_ just as in C and C++.
- The `main` function is the program entry point.
- Rust has "Hygienic macros" for example `println!`.
- Rust strings are encoded using UTF-8 and can contain Unicode characters.

## Rust advantages

This are some of the rust advantages:

- Memory safety on compile time: Avoiding all kind of memory related error on compilation

  - No uninitialized variables
  - No double free errors. -> Research laters
  - No use-after-free. -> Research later
  - No NULL pointers
  - No race conditions between threads
  - No iteratr invalidation

- There is no undefined behaviors on runtime, meaning a rust instruction cannot be omited

  - Array access point validation
  - Integer Overflow (Panic or wrap around)

- Modern day language characteristics: It is as expressive and erconomic as higher level languages
  - Enums
  - Generics
  - No overhead FFI
  - Excelent Error handling

## Variables

Rust Provides type safety via static typing. Variable Bindings are made with `let` keyword

```rs
fn main(){
	let x: i32 = 10;
	println!("X: {x}");
}
```

Variables are inmutable by default, to convert them to mutable elements you need to add the `mut` keyword in the variable declaration

```rs
fn main(){
	let mut x: i32 = 10;
	println!("x: {x}");
	x = 20;
	println!("x: {x}");
}
```

## Data types

### Signed Integers

> Signed integers are numbers that can be negative

- i8#
- i16
- i32
- i64
- i128
- isize

Examples: `-10` `0` `1_000` `123_i64`

### Unsigned integers

> Unsigned integers can only be positive

- u8
- u16
- u32
- u64
- u128
- usize

Examples: `0` `123` `10_u16`

### Floating numbers

- f32, f64

Examples `3.14` `-10.0e20` `2_f32`

### Unicode scalar values

- char

Examples: `a`

### Booleans

- bool

Examples: `true` `false`

### Data Type Sizing

- `iN`, `uN`, and `fn` have N bits of size
- `isize` and `usize` have the size of the pointer
- `char` has a size of 32 bits
- `bool` has a size of 8 bits

> Every underscore on numbers can be ommited, they are used to simplify reading. Therefore `1_000` can be writen as `1000`(Or, for some reason `10_00`) and `123_i64` can be written as `123i64`

## Arithmethics

```rs
fn interproduct(a: i32, b: i32, c:i32) -> i32 {
	return a * b + b * c + c * a;
}


fn main(){
	println!("Resultado: {}", interproduct(120, 100, 248));
}
```

Arithmethics are pretty similar to other programming languages.

What happens when there is a integer overflow? In C y C++, integer overflows with signed numbers is not defined. In Rust it is defined.

If on the previous example we change the i32 for an i16 integer we would get an error on the debugging version of the compiling/running of the code, but on a compiled version if would return the result

## Strings

In rust there are two data types to handle strings. Both types of data hold UTF-8 encoded strings

- `String` - A modifiable, owned string
- `&str` - Readonly type of string. String literals for under this category

```rs
fn main(){
	let greeting: &str = "Saludos";
	let planet: &str = "🪐";
	let mut sentence = String::new();
	sentence.push_str(greeting);
	sentence.push_str(", ");
	sentence.push_str(planet);
	println!("Frase final: {}", sentence);
	println!("{:?}", &sentence[0..5]);
	// This following line throws an error given that it doesn't end on the boundary of the
	// character
	// println!("{:?}", &sentence[12...13]);
	// A fixed version would be
	// println!("{:?}", &sentence[9...13]);
}
```

`String`is a data type defined by the user using the constructor(`::new()`) and methods like `s.puch_str(..)`.

The `&` in `&str` indicates that this is only a reference. On the time being it just means a readonly string.

## Arithmethic

Arithmetic operations work similar to other languages

## Type Inference

Rust will look at how the variable is used to determine the type.
