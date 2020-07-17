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

