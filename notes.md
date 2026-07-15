# Learning Rust
video url: https://youtu.be/rQ_J9WH6CGk?si=uReG1P6X4ODyNTdu

## Hello World Program

- functions are declared with `fn` 
- all programs must have a `fn main()`
- `rustc` then file to compile
- you can start new rust project with `cargo new project_name`, kinda like `npm init`
- `cargo run` within the project root will compile and run the project
- if single file, every time code is updated it must be recompiled with `rustc`
- however with cargo, `cargo run` will recompile it automatically

## Primitive Data Types

-statically typed
-int, float, bool, char
-Integer
    - Rust has signed and unsigned (+ and -) integers and of different sizes
    - i8, i16, i32, i64, i128: Signed integers
    - u8, u16, u32, u64, u128: Unsigned integers
-Range of i32 is from 2^31 postive to 2^31 negativej
-floats represent numbers with fractional parts
    -f32 and f64

## Compound Data Types
-arrays, tuples, slices, strings (slice strings)
-arrays: collection, fixed size, of elements of the same type
-tuples: collection, fixed size, of elements of different types

## Stuff to Remember
- `println!` requires `!` (it's a macro)
- formatting options:
    - `{}` -> Display
    - `{:?}` -> Debug
    - `{:#?}` -> "pretty" Debug
