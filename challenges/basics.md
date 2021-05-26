# Basic Challenges

## 1. Convert temperatures between Fahrenheit and Celsius.

## 2. Write a function to find factorials for a given n

## 3. Write a function to generate 'n' numbers from fibonacci series

## 4. Prime Numbers

- Loop through the numbers from 2 to 25 and print out which numbers are prime, and for those numbers which are not prime numbers, you should print them as a product of two factors
- Remember that prime = no divisors other than 1 and itself
- Don't worry about efficiency, but if you're interested, check out `f64::sqrt()`

```txt
2 is prime
3 is prime
4 is product of 2 * 2
...
```

### Extra Challenge

- Create a function which can return the products given a number
  - Think about the return value in case the given number is prime?
- DRY-up your logic

## 5. Print the lyrics to the [Beer song](https://en.wikipedia.org/wiki/99_Bottles_of_Beer)

Note that not all verses are identical.

## 6. FizzBuzz challenge

Print numbers 1 to 100:

- for every number divisible by 3 print "Fizz" instead
- for every number divisible by 5 print "Buzz" instead
- for numbers divisible by 3 & 5 print "FizzBuzz"
- for other numbers print them as is

### Extra Challenge

- Create a function `fizzBuzz` which takes a `i32` and returns a `&str`
- Ensure that you define the logic in a separate module.

## 7. Input from user

Have the user enter a string, then loop through the string to generate a new string in which every character is duplicated, e.g., "hello" => "hheelllloo".

Test with "世界" as input.

## 8. Exercism - [Sum of multiples](https://github.com/AgarwalConsulting/Rust-Training/tree/master/exercises/exercism/sum-of-multiples)

## 9. Create a `Adder` type with `add` & `result` method on it which can keep track of the total

`add` method takes a number as an argument
