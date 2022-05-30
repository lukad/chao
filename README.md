# chao

A simple lisp.

## Builtins

| Form     | Description                | Example               |
|----------|----------------------------|-----------------------|
| `lambda` | Define anonymous functions | `(lambda (x) (* x x)` |
| `set`    | Define variables           | `(set 'foo 42)`       |
| `if`     | Conditional evaluation     | `(if true ":D" "D:")` |
| `+-*/`   | Artithmetic operators      | `(+ 1 (/ 4 2.0))`     |
| `=`      | Comparison                 | `(= 1 2)`             |

## Building

1. [Install rust](https://www.rust-lang.org/en-US/install.html)
2. `git clone https://github.com/lukad/chao.git`
3. `cd chao`
4. `cargo build --release`
