# Smart Pointers

Smart pointers are data structures that not only act like a pointer but also have additional metadata and capabilities.

We’ll cover the most common smart pointers in the standard library:

- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time.

Smart Pointers usually implement the `Deref` & `Drop` traits.

You can create your own smart pointers too!

## Using `Box<T>` to Point to Data on the Heap

Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.

But they don’t have many extra capabilities either.

You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size

- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so

- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### Enabling Recursive Types with Boxes

Looking at a `Cons` List!

## `Rc<T>`, the Reference Counted Smart Pointer

There are cases when a single value might have multiple owners.

For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.

Rust has a type called `Rc<T>`, which is an abbreviation for reference counting, to enable multiple ownership. The `Rc<T>` type keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

### Usage

- `Rc::new`
- `Rc::strong_count`
- `Rc::clone`
- `Rc::downgrade`
- `Rc::weak_count`
- `.upgrade`

## `RefCell<T>`

*Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.

### Borrowing Rules

- At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
- References must always be valid.

The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the development process, and there is no impact on runtime performance because all the analysis is completed beforehand.

The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, whereas they are disallowed by the compile-time checks.

Static analysis, like the Rust compiler, is inherently conservative. Some properties of code are impossible to detect by analyzing the code: the most famous example is the Halting Problem.

### A common way to use `RefCell<T>` is in combination with `Rc<T>`

Recall that `Rc<T>` lets you have multiple owners of some data, but it only gives immutable access to that data. If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate!
