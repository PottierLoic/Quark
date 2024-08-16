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
```

# Roadmap

## Core Features
- [ ] **Types**
  - [ ] `int`
  - [ ] `float`
  - [ ] `string`
  - [ ] `bool`
- [ ] **Arrays**: Declaration, access, iteration, and operations
- [ ] **Pipe Operator**: Example - `let result = data |> filter(e -> e > 10) |> map(e -> e * 2) |> sum()`
- [ ] **Compound Assignment**
  - [ ] `+=`
  - [ ] `-=`
  - [ ] `*=`
  - [ ] `/=`
- [ ] **Logical Operators**
  - [ ] `&&`
  - [ ] `||`
- [ ] **Bitwise Operators**
  - [ ] `&=`
  - [ ] `|=`
- [ ] **Increment/Decrement**
  - [ ] `++`
  - [ ] `--`
- [ ] **Control Flow**:
  - [ ] **For Loop**: `for i in 0:10 ->`, `for item in list ->`
  - [ ] **While Loop**: `while cond ->`
  - [ ] **If-Else**: `if cond -> ... else ->`
  - [ ] **Ternary**: `let result = cond ? "True" : "False"`

## Enhancements
- [ ] **C Interop**: Potential integration with C functions/libraries
- [ ] **Error Handling**: Structured error handling for better debugging
- [ ] **Type Inference**: Automatic type deduction to simplify code
- [ ] **Pattern Matching**: Implement `match-case` constructs

## Future Considerations
- [ ] **Modules**: Organize code into modules with import support
- [ ] **Functional Programming**: Higher-order functions, lambdas, closures
- [ ] **Optimizations**: Improve code efficiency through compiler optimizations
