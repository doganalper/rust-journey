fn main() {
    generics();
    // traits();
    // lifetimes();
}

fn lifetimes() {
    /*
    Every reference has a `lifetime`, by default it is the scope for which that reference is valid. Most of the time lifetime of a reference is inferred. We must annotate lifetimes when the lifetimes of references could be related in a few different ways.

    Lifetime annotations don't change how long any of the references live. They describe the relationships of lifetimes to each other.

    Lifetime syntax is usually ` 'a `.
    -> &i32 -- a refernece
    -> &'a i32 -- a reference with an explicit lifetime
    -> &'a mut i32 -- a mutable reference with an explicit lifetime
    */

    // DANGLING REFERENCE
    /*
    Main aim of lifetimes is to prevent dangling references. It is when a variable has the reference of a value that is invalid.

    Here we would get an error since r has the reference of x which will be invalid on scope end. R will be looking deallocated place and that would cause issues. Rust prevents this by `borrow checking` meaning it compares scopes and warns us if anything is wrong.
    */
    // fn dangling_reference() {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r: {}", r);
    // }

    // This function will give error because `borrow checker` wouldn't know which reference would be returned.
    // fn longest(str1: &str, str2: &str) -> &str {
    // This is fixed function definition.
    // Using lifetime annotation in return statement means that, returned reference will be valid as long as reference with smallest lifetime is valid
    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }

    let s1 = String::from("long string is long");

    {
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {}", result);
    }

    // LIFETIME ANNOTATIONS IN STRUCTS
    // Using lifetimes with structs means that an instance created with this struct is valid as long as reference a field holds is valid.
    struct ImportantExceprt<'a> {
        part: &'a str,
    };

    let novel = String::from("Call me Mufasa. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.' ");
    let i = ImportantExceprt {
        part: first_sentence,
    };

    // STATIC LIFETIME
    /*
    Static lifetime means that a reference can live for the entire duration of the program.
    */
}

fn traits() {
    // DECLARING A TRAIT
    pub trait Summary {
        // Each type that implements this trait must have a method called summarize with given method signature
        fn summarize(&self) -> String;

        // This is called default implementation of a method
        // If a struct wants to override this method it still needs to follow method signature
        fn read_mode_text(&self) -> String {
            String::from("Read More...")
        }

        // A default implementation can call other methods from same trait
        fn summarize_and_read_more(&self) -> String {
            format!("{} and {}", self.summarize(), self.read_mode_text())
        }
    }

    // IMPLEMENTING A TRAIT
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{} by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // impl <Trait_Name> for <Struct_Name>
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {} ", self.username, self.content)
        }
    }

    impl Tweet {
        fn get_retweet_count(&self) -> u64 {
            4
        }
    }

    // We can use traits as parameters to expand and narrow possible structs function may take as parameter
    // Here we don't need to know specific struct item may be but we are saying it must have methods that Summary defines
    // This also can be written like this:
    // pub fn notify<T: Summary>(item: &T) {}
    // writing it like this is useful when there are multiple parameters that needs to implement a trait
    // pub fn notify<T: Summary>(item: &T, item2: &T, item3: &T) {}
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    trait Display {}
    // We can also specify more than one trait bound.
    // This also can be written like this:
    // pub fn notify_multiple<T: Summary + Display>(item: &T) {}
    pub fn notify_multiple(item: &(impl Summary + Display)) {}

    trait Debug {}
    // Clear way of writing multiple parameters with multiple trait bounds.
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    // This is way complicated to read. It can be simplified like this:
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        4
    }

    // Just like accepting a trait for parameter type, functions may return a type that implements traits
    // But due to how traits implemented in compiler, functions that returns a trait must have only one return type
    // So branching on this function and returning another struct that implements Summary trait depending on a condition
    // would result in error at compile time.
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // Traits defines a functionality a type has and can share with other types.
    // In most ways traits are interfaces in other languages but has some differences.

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // When calling a method implemented from trait, if they are on different file, we must both call Tweet type and Summary trait.
    println!("1 new tweet: {}", tweet.summarize());

    // Passing struct that implements a trait
    // We can use tweet as parameter since it implements Summary trait
    notify(&tweet);
}

fn generics() {
    /*
    Generics, abstract stand-ins for concrete types or other properties.
    Example: Functions can take parameters with some generic type, instead of concrete types. We may want to do some calculations on an array without depending of types of it holds.
    */

    fn largest_i32_old(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item
            }
        }

        largest
    }

    fn largest_char_old(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item
            }
        }

        largest
    }

    // GENERICS IN FUNCTION DEFINITIONS
    // Using `std::cmp::PartialOrd` to narrow possible types this function may take by only accepting types that can be compared
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item
            }
        }

        largest
    }

    // GENERICS IN STRUCT DEFINITIONS
    struct Point<T> {
        x: T,
        y: T,
    }

    // We can define multiple generic type
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    // GENERICS IN ENUM DEFINITIONS
    enum Option<T> {
        Some(T),
        None,
    }

    enum Resutl<T, E> {
        Ok(T),
        Err(E),
    }

    // GENERICS IN METHOD DEFINITIONS
    // This will be available on every Point instance regarding of concrete type
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // We can also define some other methods for specific concrete type
    // This will only be available if passed concrete type is f32
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32_old(&number_list);

    let char_list = vec!['y', 'a', 'm', 'q'];
    let result = largest_char_old(&char_list);

    // As we can see from this example, identical code needed to be duplicated in order to work with two different types.
    // Generics is used to get rid of this duplication as we can expand possible types a function may take.
    // By convention type parameter names are short and commonly used with `T`.
    // see line 22

    // We can also use generics in structs
    let integer_point = Point { x: 4, y: 10 };
    let float_point = Point { x: 4.2, y: 5.7 };
    // let mixed_point = Point {x: 4.2, y: 10}; // This would not work since type of both x and y must be same type

    let fixed_mixed_point = MixedPoint { x: 4, y: 5.7 }; // This will work since on mixed point we accept two generic type and use them types of x and y
    let integer_mixed_point = MixedPoint { x: 4, y: 4 }; // This also works because both generic types can have same type

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let pf = Point { x: 5.4, y: 2.4 };
    // distance_from_origin method can be called because concrete type of Point is f64
    println!("distance_from_origin: {}", pf.distance_from_origin());
}
