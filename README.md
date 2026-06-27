# chao

A small Lisp interpreter.

## Syntax

- Values: `nil`, booleans, integers, floats, strings, symbols, and lists.
- Quote: `'expr` returns `expr` as data.
- Quasiquote: a backtick-prefixed expression returns an expression template.
- Unquote: `,expr` evaluates `expr` inside a quasiquote.

## Forms

| Form       | Description                    | Example                                            |
|------------|--------------------------------|----------------------------------------------------|
| `lambda`   | Create an anonymous function   | `(lambda (x) (* x x))`                             |
| `def`      | Define a variable              | `(def answer 42)`                                  |
| `def`      | Define a function              | `(def square (x) (* x x))`                         |
| `set`      | Assign an existing binding     | `(set answer 43)`                                  |
| `if`       | Evaluate one branch lazily     | `(if true "yes" "no")`                             |
| `list`     | Build a list                   | `(list 1 (+ 1 1) 'x)`                              |
| `defmacro` | Define a macro                 | <code>(defmacro when (c b) `(if ,c ,b nil))</code> |
| `intern`   | Convert a string to a symbol   | `(intern "name")`                                  |
| `+ - * /`  | Arithmetic operators           | `(+ 1 (/ 4 2.0))`                                  |
| `= < >`    | Comparison operators           | `(= 1 2)`                                          |

`def` creates or replaces a binding. `set` updates an existing binding and errors when the name is unbound.

## Macros

Macros receive raw syntax and return an expression that is evaluated in the caller's environment.

```lisp
(defmacro when (cond body) `(if ,cond ,body nil))
(when true (+ 1 2))
```

## Building

1. [Install rust](https://www.rust-lang.org/en-US/install.html)
2. `git clone https://github.com/lukad/chao.git`
3. `cd chao`
4. `cargo build --release`
