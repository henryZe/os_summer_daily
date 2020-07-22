# 1st Chapter

## 1 Hello World

1. `println!` is a [*macro*](https://doc.rust-lang.org/rust-by-example/macros.html) that prints text to the console.

2. using the Rust compiler: `rustc`

### 1.1 Formatted print

* `format!`: write formatted text to [`String`](https://doc.rust-lang.org/rust-by-example/std/str.html)
* `print!`: same as `format!` but the text is printed to the console (io::stdout).
* `println!`: same as `print!` but a newline is appended.
* `eprint!`: same as `format!` but the text is printed to the standard error (io::stderr).
* `eprintln!`: same as `eprint!`but a newline is appended.

## 2 Primitives

* Scalar Types
    * signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    * unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    * floating point: f32, f64
    * char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    * bool either true or false
    * and the unit type (), whose only possible value is an empty tuple: ()
