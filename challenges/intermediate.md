# Intermediate

## 1. Mars Rover

A squad of robotic rovers are to be landed by NASA on a plateau on Mars.

This plateau, which is curiously rectangular, must be navigated by the rovers so that their on board cameras can get a complete view of the surrounding terrain to send back to Earth. A rover's position is represented by a combination of an x and y co-ordinates and a letter representing one of the four cardinal compass points. The plateau is divided up into a grid to simplify navigation. An example position might be 0, 0, N, which means the rover is in the bottom left corner and facing North.

In order to control a rover, NASA sends a simple string of letters. The possible letters are 'L', 'R' and 'M'. 'L' and 'R' makes the rover spin 90 degrees left or right respectively, without moving from its current spot. 'M' means move forward one grid point, and maintain the same heading. Assume that the square directly North from (x, y) is (x, y+1).

Input (whether hard coded or input from keyboard): The first line of input is the upper-right coordinates of the plateau, the lower-left coordinates are assumed to be 0,0. The rest of the input is information pertaining to the rovers that have been deployed. Each rover has two lines of input. The first line gives the rover's position, and the second line is a series of instructions telling the rover how to explore the plateau.

The position is made up of two integers and a letter separated by spaces, corresponding to the x and y co-ordinates and the rover's orientation. Each rover will be finished sequentially, which means that the second rover won't start to move until the first one has finished moving. Output: The output for each rover should be its final co-ordinates and heading.

Plateau max X and Y, Starting coordinates, direction and path for two rovers:

```txt
5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM
```

Output and new coordinates:

```txt
1 3 N
5 1 E
```

## 2. Vectors

Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

## 3. Strings

Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

## 4. HashMap

Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

## 5. Exercism - [Sum of multiples](https://github.com/AgarwalConsulting/Rust-Training/tree/master/exercises/exercism/sum-of-multiples)

## 6. Exercism: [Sublist](https://github.com/AgarwalConsulting/Rust-Training/tree/master/exercises/exercism/core/sublist)

## 7. Exercism: [Space Age](https://github.com/AgarwalConsulting/Rust-Training/tree/master/exercises/exercism/core/space-age)

## 8. Exercism: [Luhn](https://github.com/AgarwalConsulting/Rust-Training/tree/master/exercises/exercism/core/luhn)

## 9. Implement your own `filter` function

  A filter function will take a slice & a predicate which evaluates to `bool`. The filter `fn` should return a slice back of same type.

Eg. Take a vec! of string: `vec!("Iron Man", "Batman", "Superman", "Spider-man", "Wonder Woman", "Iron Fist", "Daredevil", "Supergirl", "Flash")`.

Some predicate examples...

- Display all heroes whose name's second character is a vowel (a, e, i, o u)
- Display all heroes whose name doesn't contain "man" in it

## 10. Fibonacci closure

  Implement a fibonacci function that returns a function (a closure) that returns successive fibonacci numbers (0, 1, 1, 2, 3, 5, ...).
