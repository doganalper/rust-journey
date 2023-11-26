fn main() {
    // variables_and_mutability();
    // data_types();
    // functions();
    control_flow();
}

fn control_flow() {
    // IF EXPRESSIONS
    let num = 3;

    // branches of if / else if / else are sometimes called arms (just like in `match` expression)
    // conditions always must be a bool.
    if num < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    let num2 = 6;

    if num2 % 4 == 0 {
        println!("number is divisible by 4")
    } else if num2 % 3 == 0 {
        println!("number is divisible by 3")
    } else if num2 % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4,3 or 2")
    }

    // Since if is an expression it can be used on the right side of `let`
    // Here since values inside curly brackets have no semicolons they evaluate as expression and can be used here.
    // Both values on if and else must have same type.
    let number = if true { 5 } else { 6 };

    // LOOP
    // loop block will run forever until program explicitly tell it to stop or user stops it with ctrl-c
    // way of breaking loop is using `break` keyword inside of a condition
    // you can also skip current loop depending on condition by using keyword `continue`
    // one use of loop is to retry an operation you know might fail.
    // loop can be an expression by adding a value that you want to return after `break` keyword
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // this section will be evaluated and return it to given variable
        }
    };

    println!("The result is: {result}");

    // It is also possible to label loops and `break` and `continue` specific loops by using labels.
    // this is really useful when using nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break; // this will break current loop
            }

            if count == 2 {
                break 'counting_up; // this will break outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // WHILE
    // while runs unless given condition evaluates to true
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("While ended.");

    // FOR
    // a way to loop through an array and access each element is using for loop
    let arr1 = [10,20,30,40,50];

    for element in arr1 {
        println!("the current value is: {element}");
    }

    // you can also use for loop to do something for amount of times you want
    // this can be done using Range, provided by the standart library
    // here 4 will be excluded so given range starts from 1 and ends on 3
    // to include 4 you can use `..=` syntax
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("For ended.");
}

fn functions() {
    // Rust uses `snake case` as the conventional style for function and variable names.
    // function can be defined with `fn` keyword
    // Parameters of functions need to have defined type.
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to resultant value and returns it.
    // Function definitions are statements, it does something
    // Function callings are expressions, macro callings are expressions and new scope block created with curly brackets are an expression.

    // Here `x + 1` does not have semicolons at end, this means it is an expression.
    // If semicolon is added it would be a statement
    // Add semicolon and see how type of y is changing when semicolon is not added.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Functions can return values where they are being called.
    // Return type of a function must be annotated by `-> <return_type>`
    // Return value of the function is the final expression of the function body.
    // `return` keyword can be used to early return in function.

    fn example_function() -> i32 {
        4 // this is what example_function will return because it is an expression
    }
    let example_variable = example_function();

    println!("Example variable is: {example_variable}");
}

fn data_types() {
    // Rust is a statically typed language and it must know the types of all variables at compile time.
    // Usually compiler can infer what type we want the variable to hold based on the assignment.
    // But on cases that have possibly multiple types, we must declare the type ourselves.
    // This was the case when we converted string value to number type.
    // Without adding type annotation program wouldn't compile.
    let guess: u32 = "42".parse().expect("Not a number!");

    scalar_types();
    compound_types();
}

fn compound_types() {
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types
    // * Tuple
    // * Arrays

    // Tuple Type
    // Tuples are data types that can store multiple values with fixed length, once declared they are not able to
    // grow or shrink in size.
    // Values inside a tuple can have different types.
    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    // We can destructure a tuple value with pattern matching
    // Since we use pattern matching here, compiler will give an error if we haven't destructure a value in tuple
    // We can omit unwanted values from destructuring by giving it a name as underscore
    let (x, y, z) = tup1;

    // Tuple values can be individually reachable by index (starts from 0)
    let five_hundred = tup1.0;
    let six_point_four = tup1.1;
    let one = tup1.2;

    // Tuples without any value is called `unit`. This are useful on empty value or empty return type.
    // Both type and value is created by empty parantheses `()`
    let empty_tup: () = ();

    // Array Types
    // Another way of storing multiple values are arrays.
    // Arrays must contain values with same types.
    // Arrays have fixed length, same as tuples.
    // Arrays are allocated on the stack rather than the heap
    // Type annotation of array firstly declares type of values it will hold and secondly size of array.
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];

    // It is also possible to create an array containing same values with given length
    let arr2 = [3; 2]; // arr2 will have a value [2,2]

    // To access value inside of an array we can call it with it's index
    let first = arr1[0];
    let second = arr1[1];
}

fn scalar_types() {
    // SCALAR TYPES
    // It represents single value.
    // Rust has 4 primary scalar type: integers, floats (floating-point numbers), Booleans and characters.

    // INTEGER TYPES
    // It is a number without fraction. It can have both signed and unsigned values. (Signed means it can be both positive or negative) Signed integer type starts with `i` (i32) and unsigned starts with `u` (u32)
    // If integer overflow happens (integere assignment above or below of declared size), on debug compile it will panic at runtime, on non debug compile it will not panic at all. Instead variable will have an unwanted value.
    let i1 = 24;
    let i2: i64 = 48;

    // FLOATING-POINT TYPES
    // Rust has two types for floating-point numbers. One is `f32` other is `f64`.
    // All floating-point types are signed.
    // Default type is `f64` since modern on CPUs it has the same speed as `f32` but is capable of more precision.
    let f1 = 2.0;
    let f2: f32 = 3.0;

    // BOOLEAN TYPE
    // It can have two value `true` or `false.
    // Booleans are one byte in size.
    let t = true;
    let f = false;

    // CHAR TYPE
    // It is a Rust's most primitive alphabetic type.
    // It is declared with single quotes, as opposed to string literals that use double quotes.
    let c = 'c';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
}

// By default variables are immutable. This behaviour can be changed by adding `mut` keyword when defining a variable.
fn variables_and_mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    // Defining constants:
    // Constants are always immutable variables. Meaning `mut` keyword will not work with them.
    // When defining type annotation must be done.
    // Value of a constant must be a constant expression, meaning it cannot be computed at runtime.
    // Naming convention of Rust on constants is it should be all uppercase with underscores between words.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    // When a variable with previously declared name is declared, this is called shadowing. Recently created variable shadows previously created variable with same name.
    // Shadowing only affects same scope, any shadowing done under new scope only affects that scope.
    let y = 5;

    let y = y + 1; // y becomes 6 here

    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}") // y is 12 here
    }

    println!("The value of y is: {y}") // y is 6 here
}
