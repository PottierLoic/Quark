# Quark Programming Language

Quark is a programming language designed as a training project to explore language design and implementation. This project is built using Rust.

## Example

Here’s a simple example of a Quark program that WOULD calculates the Fibonacci sequence:

```quark
fnc fibonacci(n int) int ->
  ret match n ->
    | 0 -> 0
    | 1 -> 1
    | _ -> fibonacci(n - 1) + fibonacci(n - 2)
end

fnc main() int ->
  let result = fibonacci(10)
  print(result)
  ret 0
end
