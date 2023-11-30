use std::arch::aarch64::vld1_f32;

fn main() {
    // Ownership is a set of rules that govern how a Rust program manages memory.
    // Some languages have garbage collector, others leave memory management to developer to allocate memory.
    // Rust have different approach for this, memory is managed with a system of ownership with set of rules that the compiler checks.
    // On situations where these rules are violated, the program will not compile.

    // Storing values on the stack or the heap affects performance of program.
    // THE STACK
    // Stack stores values in the order it gets them and removes in opposite order. (Last in, first out)
    // Adding data is called pushing onto the stack and removing is popping off the stack.
    // All data stored on the stack must have a known fixed size.

    // THE HEAP
    // It is less organized comparing to the stack
    // When put data on the heap, memory allocator finds an empty spot in the heap that have enough space, marks it as being in use and returns a pointer (address of that location).
    // Putting data is called allocating on the heap (or allocating)
    // Pointer has a known fixed size and so it can be stored on the stack

    // Pushing on the stack is faster since it does not have to search for safe space unlike allocating on the heap
    // Accessing data in the heap is slower since you have to follow a pointer to get there.

    // OWNERSHIP RULES
    // * Each value in Rust has an owner.
    // * There can only be one owner at a time.
    // * When the owner goes out of scope, the value will be dropped.

    // variable_scope();
    // string_type();
    // variables_and_data_interacting_with_move();
    // variables_and_data_interacting_with_clone();
    // stack_only_data();
    // ownership_and_function();
    // return_values_and_scope();

    // references_and_borrowing();
    // dangling_references();

    slice_type();
}

fn slice_type() {
    // this function takes a reference and returns index of last char of the word
    // if given string has no spaces it means it is only one word and can return length of the word
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("hello world");

    let word = first_word(&s); // word would be 5 since index of space is 5

    s.clear(); // this empties the string, s would be ""

    // after clearing s, index in word variable will not be usefull since size of s is 0 and 5 is invalid index
    // a way of solving this problem is using `string slice`, it is a reference to part of a `String`

    let mut s = String::from("hello world");
    // hello and world is a reference to portion of `String` s
    // string slice can be created using range syntax, since ending index is not inclusive we must define it with on extra number
    // another way of starting range from zero is by omitting start index [..5] would result in same
    // by omitting ending index we are saying to take until the end of string
    // string slice expression returns static string type
    let hello = &s[0..5];
    // here we included ending index so we didn't add extra number
    let world = &s[6..=10];

    fn first_word_fixed(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
        }

        &s[..] // this means start index is 0 and ending index is last index
    }

    let mut s = String::from("hello world");

    let word = first_word_fixed(&s);

    // s.clear(); // this will throw an error since word is being used after here and that means immutable borrow didn't go out of scope and hence we can't borrow mutable reference

    println!("first word: {word}");

    // Type of s is &str, it is a slice pointing to that specific point of the binary. It is immutable since &str is an immutable reference.
    let s = "Hello, world";
    // by converting our function signature to someting like `fn first_word(s: &str) -> &str {}`
    // we can now use this function with string slices (both &str and Slice)
    // we can pass string literals directly since they are string slices
    // and we can pass String values by referencing to it's slice by &s[..] or directly &s

    // we also can have slice of array
    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);
}

fn dangling_references() {
    // let reference_to_nothing = dangle();

    // This would be problem since s will be out of scope and Rust would not allow us to return a reference of variable that will be dropped.
    // This problem can be solved by changing return value type to &'static String
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }
}

fn references_and_borrowing() {
    // Instead of taking and giving ownership with function
    // another way of doing something in a function is providing a reference to the value.
    // A reference is like a pointer in that it's an address we can follow to access the data stored at that address.
    // Creation of a reference is called `borrowing`
    let s1 = String::from("hello");
    // & sign here means reference of s1 value
    let len = calculate_length(&s1);

    println!("The length of {s1} is {len}");

    // & here means it will be reference of String type
    fn calculate_length(s: &String) -> usize {
        s.len()
    } // here s will be dropped since it goes out of scope but reference that passed to this function will not dropped since ownership of it does not go out of scope

    // Borrowed values cannot be modified. They are immutable by default
    // They can be done mutable by adding `&mut`
    // str2 has a type of mutable reference of String type
    // There's one big restriction with mutable reference, you can have no other references to that value.
    // Creating multiple mutable reference would fail.
    // Reason of this restriction is to prevent `data race` where multiple reference trying to access or mutate same value
    // Way of creating multiple reference is by creating one in different scope
    // str has a type of reference of String type and is immutable
    fn change(str: &String, str2: &mut String) {
        // str.push_str(", world"); // this would give an error and code would not compile
        str2.push_str(", world"); // this would not give an error since reference set as mutable
    }

    // Having immutable and mutable references are problem if immutable reference is not used. 
    // Reason for this is after the last use of variable it goes out of scope. On this example after printing immutable borrows they both go out of scope and mutable borrowing become possible.
    // Example:
    let mut s2 = String::from("hello 2");
    let r1 = &s2; // no problem
    let r2 = &s2; // also no problem here 
    // let r3 = &mut s2; // this would be problem because immutable borrows are no used before mutable borrowing
    println!("{} and {}", r1, r2);
    let r3 = &mut s2; // this would not be problem because immutable borrows are used before mutable borrowing
}

fn return_values_and_scope() {
    // returning values can also transfer ownership.

    // this function will move its return value into the variable that calls it
    fn gives_ownership() -> String {
        let some_string = String::from("gives_ownership");

        some_string
    }

    // this function takes an ownership and mutates it and later gives ownership to the variable that calls it
    fn takes_and_gives_back(mut a_string: String) -> String {
        a_string.push_str(", this added inside");

        a_string
    }

    let s1 = gives_ownership();

    let s2 = String::from("s2 value");

    let s3 = takes_and_gives_back(s2);

    println!("s1: {s1}");
    println!("s3: {s3}");
} // here s1 and s3 goes out of scope and they are dropped. s2 is moved so nothing happens.

fn ownership_and_function() {
    // Passing a variable to a function will move or copy, same as assignment does.

    let s = String::from("hello"); // s enters to scope

    takes_ownership(s); // s's value moves into the function, so it is not valid in this scope.

    let x = 5; // x enters to scope
    makes_copy(x); // x is being copied and passed to the function since i32 is Copy

    fn takes_ownership(some_string: String) {
        // some_string enters scope
        println!("{some_string}")
    } // some_string goes out of scope and `drop` is called

    fn makes_copy(some_integer: i32) {
        // some_integer enters scope
        println!("{some_integer}")
    } // some_integer goes out of scope

    // s is invalid here
    // x is valid here and after this scope ends it is being dropped
}

fn stack_only_data() {
    // Values that are stored in the stack (example: integers) can be copied by value easily.
    // No need to use `clone` method since they are inexpensive to clone.

    // There's `Clone` trait and if a type implements the `Copy` trait, variables that use it can be cloned easily, they don't move but rather trivially copied.
    // If a type or any part of it implements `Drop` trait, Rust won't let us unnotate a type with `Copy` trait.
    // Types that implements `Copy` trait are:
    // * All integers
    // * Boolean type
    // * Floating-point types
    // * Char
    // * Tuples if they contain types that does not implement `Drop` trait (example: (i32, String) does not implement `Clone`)
}

fn variables_and_data_interacting_with_clone() {
    // In situations where we want to copy head data of the `String`, not just the stack data
    // `clone` method can be called.
    // here s1 and s2 looks for different places on the heap
    // this shouldn't be common implementation since it can be expensive.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {s1}, s2: {s2}");
}

fn variables_and_data_interacting_with_move() {
    // Values on stack are copied by value by default.
    // This means y clones the value of x and any change on x will not mutate the value on y
    let x = 5;
    let y = x;

    // Normally s1 and s2 would look to the same pointer when copied.
    // This is called copy by reference and when trying to free memory on the end of this scope, this would be problem
    // because since two variables have same reference when memory place is dropped on one variable, it would be problem for second variable to drop since place on the pointer is empty at that point (this is called double free error)
    // instead in Rust, when value on the heap is clonned ownership of the place moves to the new variable.
    // this is called `move` and after that first variable is invalid.
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("s1: {s1}"); // this would throw an error since new owner of the value is s2 and s1 is invalid.
}

fn string_type() {
    // String type is stored on the heap and unlike previously seen data types, it has no known fixed size.
    // `s` variable on `variable_scope` is called string literals and they are immutable.
    // String literals are not useful on situations like where we might not know it's value on compile time (example: user inputs)
    // For situations like this Rust has another string type. This type manages data allocated on the heap and don't have known fixed size.
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{s}");

    // In order to support a mutable, growable piece of text with `String`, we need to allocate an amount of memory on the heap. With this:
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we are done with `String`

    // By calling `String::from`, we request a place in the heap.
    // Second part is done by rust, when variable with requested space in the heap goes out of scope it is being returned to the memory.
    // When variable goes out of scope, Rust calls a function called drop automatically.
}

fn variable_scope() {
    // s is not valid, not yet declared
    let mut s = "hello"; // from this point s is valid

    s = "hello world";

    println!("{s}");
} // after ending this scope, s is not valid
