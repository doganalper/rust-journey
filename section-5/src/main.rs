// In a way structs are like tuples. They can store multiple related data with different types.
// What differs them is that in structs you can name these variables. It is safer way to access data inside of them.

// This is how to create structures.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // struct_basics();
    // basic_program_with_structs();
    // method_syntax();
}

fn method_syntax() {
    // methods are similiar to function. Unlike functions, methods are defined within the context of a struct (enum, trait objects).
    // first parameter of methods are always `self` that represents the instance of the struct the method is being called on.
    // lets do previous example with methods

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // anything inside of impl with struct name will be associated with that struct.
    // area method now can only be used with instances of Rectangle struct
    // methods can take ownership of self, borrow self immutable or borrow self mutably.
    // taking ownership is rare implementation.
    // One structure can hold multiple `impl` blocks.
    impl Rectangle {
        // previously explained `self` as parameter
        // &self is actually short for self: &Self
        fn area(&self) -> u32 {
            // self here refers to the instance of Rectangle
            self.width * self.height
        }

        // we can name met&selfhods same names with struct's fields
        // this would not be problem since they both can be used with different syntax
        // this method is called getter
        fn width(&self) -> bool {
            self.width > 0
        }

        fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
            self.width < other_rectangle.width && self.height < other_rectangle.height
        }

        // any method that does not have self as first parameter is called associated function and not called methods at all
        // They are often used for constructors that will return a new instance of the struct.
        // Self keywords in the return type and in the body are aliases for the type that appears after `impl`
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // this is called method syntax.
    );

    if rect1.width() {
        println!("The rectangle has a nonzero with; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // Associated functions are called with `::` syntax with the struct name
    let square = Rectangle::square(3);
}

fn basic_program_with_structs() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // this would not work because Rectangle structure does not derive Display trait
    // println would print variable with type that implements std::fmt::Display trait.
    // println!("rect1 is {}", rect1);
    // Putting `:?` tells `println` macro that we want to use an output format called Debug. `Debug` trait enables us to print our stuct.
    // for that to work we must add #[derive(Debug)] to our struct
    // to prettify the output we can also use `:#?`.
    println!("rect1 is {:?}", rect1);

    // another way to print out a values using `Debug` format is the use `dbg!` macro
    // dbg! macro takes an ownership of an expression (unlike println!)
    // since it takes an ownership and variables passed to `dbg!` macro cannot be used afterwards

    // It is common usage to borrow a struct instance rather than owning it.
    fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
}

fn struct_basics() {
    // Struct or structure is a custom data type. It lets you store related values.
    // They can hold functions (called methods)
    // and variables (called properties)

    // Here we create an instance of struct
    // Order of this fields are unrelated.
    // By default instances of structs are immutable, this can be changed by adding `mut` keyword.
    // Rust does not allow for individual fields to be marked as mutable, entire instance must be marked as mutable
    let mut user1 = User {
        active: true,
        username: String::from("some_username"),
        email: String::from("mock@mail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another_mock@mail.com");

    let user2 = build_user(String::from("user2@mail.com"), String::from("username2"));

    // It is often useful to create new instance of struct with field values from existing instance.
    // Here you can use existing instance by accessing it with dot syntax: `active: user1.active`
    // shorter version of this would be `..` syntax (kinda works like in javascript)
    // any values used to create new instance from existing instance are moved to new instance,
    // that means we cannot access these values from existing instance.
    // this only happens on types that does not implements `Copy` trait.
    let user3 = User {
        email: String::from("another@mail.com"),
        ..user1
    };

    // we cannot use access username from user1 instance since on previous lines it moved to user3
    // but it is not a problem for sign_in_count since it's type implements `Copy` trait
    // println!("{}", user1.sign_in_count);
    // println!("{}", user1.username);

    // you can also create structures with tuples. they are called `tuple structs`.
    // they are good way to differentiate some tuples from other ones.
    // as is in tuples, values inside of them don't have names, only types of variables.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // these two variables are not same even though they hold same looking values.
    // a function that accepts a parameter with type of Color cannot take a variable with instance of Point type.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // another way of using structs are `unit-like` structs
    // they don't hold any fields inside of it.
    // they are useful when we want a type to implement a Trait (it is discussed on later sections)
}

// this function creates an instance of struct and returns it.
// on fields we can omit variable name if the variable and field name matches (just like in javascript). It is called `field init shorthand` syntax.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
