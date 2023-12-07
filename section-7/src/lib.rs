// Very good video source about this topic: see https://www.youtube.com/watch?v=969j0qnJGi8

// Rust has a number of features that allow developers to manage code's organization, including but not limited to which details are exposed, which are private and names in each scope in your programs.
// Packages: A Cargo feature that lets you build, test and share crates,
// Crates: A tree of modules that produces a library or executable,
// Modules and use: Let you control the organization, scope and privacy of paths,
// Paths: A way of naming an item, such as a struct, function, or module

// Packages and crates
// Crate:
// It is the smallest amount of code that compiler considers at a time.
// They can contain multiple modules, and these modules can be defined in other files that get compiled with the crate.
// It has two forms, a binary crate or a library crate.
// Binary Crate: They are programs you can compile to an executable (example: CLI program or server).
// They need to have `main` function that will start the program.
// Library Crate: They don't contain a main function and don't compile to an executable. Goal of library crates are being used on other projects. (example: `rand` crate used before). Most of the time when developers say `crate` they mean library crate and sometimes can be used interchangeably as `library`.
// Crate root is a source file that compiler starts from and makes up the root module of the crate.

// Package:
// It is a bundle of one or more crates that provide a set of functionality.
// It contains `Cargo.toml` that describes how to build those crates. Example for a package would be `Cargo`, it contains the binary crate for the CLI used to build code.
// Package can contain as many binary crates as developer needs but at most only one library crate. It must contain at least one crate, irrelevant of its type.
// If package contains `src/main.rs` it is a binary crate and if it contains `src/lib.rs` it is library crate.
// If package contains both `main.rs` and `lib.rs` under `src` folder it contains one binary and one library crate.
// A package can have multiple binary crates under `src/bin` folder. Each file under that will be a seperate binary crate.

// Modules:
// They let us organize code within a crate for readability and easy reuse.
// It also can be used to control the privacy of items. By default code within a module is private.
// Private codes can be used internally in module but not available for outside use.

// We can access front_of_house without declaring it pub because function `eat_at_restaurant` is defined on the same module.
mod front_of_house {
    // We can hoist modules inside of module.
    // Modules can hold function, structs, enums, constants and traits

    // This module is accesable for outer usage
    pub mod hosting {
        pub fn add_to_waitlist() {
            seat_at_table(); // accessing item inside of same module
                             // super refers to the parent module
            super::serving::take_order(); // accessing sibling module item through starting path from parent module
        }

        fn seat_at_table() {}
    }

    // This module is unaccesable for outer usage
    mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order(); // accesing item from same module
        super::deliver_order(); // accessing item from parent module which is the file itself in this situation
        hosting::add_to_waitlist(); // accessing item from sibling module
    }
}

mod back_of_house {
    // Making Structs and Enums public
    pub struct Breakfast {
        pub toast: String,      // this will be accessible for outer usage
        seasonal_fruit: String, // this will not be accessible for outer usage
    }

    // no need to make impl pub
    impl Breakfast {
        // this will not be accessible for outer usage
        fn make_breakfast() {}

        // this will be accessible for outer usage
        // we need this associated function to construct Breakfast because one of the fields is private we cannot use it when initializing Breakfast directly
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // making enum public will directly make its variants public, no need to define them as pub separately
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

// To direct Rust to where to find an item in a module tree, we must use paths.
// Path has two forms:
// * An `absolute path`, it is the full path starting from a crate root; external code can access it by starting path with crate name, internal code can use it by starting path with `crate` keyword.
// * A `relative path`: starts from the current module and uses `self`, `super` or an identifier in the current module.
// Both absolute and relative paths are followed by double colons to use what is inside of modules.
// // Absolute path
// crate::front_of_house::hosting::add_to_waitlist();

// // Relative path
// front_of_house::hosting::add_to_waitlist();
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // we can access to toast since it is public
    println!("I'd like {} toast please.", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // we cannot access to seasonal_fruit since it is not public
}

use front_of_house::hosting;

// instead of writing long paths both in relative or absolute path we can use `use` keyword to define shorter name to use it
// we can now use hosting directly without writing full path
// use crate::front_of_house::hosting;
// use can be called like this, where only add_to_waitlist will be ready to use easily.
// but this can be confusing and create situations like checking whether `add_to_waitlist` is defined locally or not.
// and another thing is sometimes multiple modules may have items with same names so it can cause problems.
// so most of the time first approach is prefered.
// use crate::front_of_house::hosting::add_to_waitlist;

// a way of solving `multiple modules may have items with same name` problem is using `as` keyword to give syntatic name to used item
// use std::fmt::Result;
// use std::io::Result as IoResult;
// fn function1() -> Result {
//     // ....
// }

// fn function2() -> IoResult<()> {
//     // ....
// }

pub fn eat_at_restaurant_again() {
    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist(); // this will not work because `hosting` is not defined to use in this scope.
    }
}

// using `pub` with `use` we can re-export given module.
// it is usefull when the internal structure of code is different from how other developers will call code.
// pub use crate::front_of_house::hosting;

// when using different items from same path we can use nest paths
// use std::{cmp::Ordering, io};

// sometimes we may need to use item and subitem of it at the same time.
// here we can refer item as self
// use std::io::{self, Write};

// with glob operator we can bring all public items define in a path into scope.
// use std::collections::*;

// we are declaring a module path, compiler will look for `src/front_of_house2.rs` file to use this module
mod front_of_house2;
