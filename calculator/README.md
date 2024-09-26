# Project: Command-Line Calculator

Create a command-line calculator that takes in basic arithmetic expressions as input and evaluates them. You can start with simple operations like addition, subtraction, multiplication, and division.

## Project Outline

- [x] Create a new Cargo project using `cargo new calculator`.
- [x] Define an enum to represent the different operations (e.g., Add, Subtract, Multiply, Divide).
- [x] Take in user input. 
- [x] Write a function to parse the user input (a string) into an operation and two operands (numbers).
- [ ] Implement the logic to perform the operation based on the enum value.
- [ ] Handle errors for invalid input, division by zero, etc.
- [ ] Use Cargo to build and run the program.

## Additional Features

- Support for more advanced operations like exponentiation, modulus, etc.
- Allow users to enter multiple expressions and evaluate them sequentially.
- Implement a simple memory system to store and recall previous results.

## Learning Objectives

This project should help you practice the following concepts:

- Enums and pattern matching
- Functions and function arguments
- Error handling
- Basic arithmetic operations
- Cargo and project structure

```plaintext
To parse the user input into an operation and two operands, you can follow these steps:

1. Split the input string into parts using whitespace as a delimiter.

2. Check the number of parts to ensure you have three (two operands and one operator).

3. Parse the first and third parts as numbers (f64 or i32, depending on your needs).

4. Match the second part (operator) against strings like "+", "-", "*", "/" to determine the operation.

5. Create a function that returns a tuple or custom struct containing:
   - The parsed `MathProblem` enum variant
   - The two parsed number operands

6. Use pattern matching or if-else statements to convert the operator string to your `MathProblem` enum.

7. Handle potential errors, such as:
   - Invalid number of parts
   - Failed number parsing
   - Unrecognized operator

8. Return a `Result` type to propagate any errors to the calling function.

This approach allows you to cleanly separate the parsing logic from the calculation logic, making your code more modular and easier to maintain.
```
