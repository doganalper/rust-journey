use std::net::IpAddr;

fn main() {
    // Enums (short for enumerations) allows us to define a type by enumerating its possible variants.
    // Enums gives us a way of saying a value is one of the possible values.
    // Example: Rectangle is one of the possible shapes that also may include Circle or Triangle.

    // enum_values();
    // option_enum();
    // match_control_flow();
    if_let_control_flow();
}

fn if_let_control_flow() {
    // on flows with multiple value patterns that we only want to do something on one specific pattern
    // we can use if let syntax.
    let config_max = Some(3u8);
    // here max will be the value config_max has inside of it
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(i32),
    }

    // we can use else to do something else if pattern does not match
    // this way is more rapid if we want to do something on one case instead of using match and need to handle all possible cases
    let mut count = 0;
    let coin = Coin::Quarter(4);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}

fn match_control_flow() {
    // Rust has a very powerful flow construct called `match`. With it we can compare a value agains multpile patterns and execute code based on these patterns.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // we want to compare coin variable with possible value patterns
        // since it has a enum type coin can only have patterns matching with that enum
        // unlike `if` statement, condition here does not need to evaluate to boolean
        // every value pattern we want to match is called match arms
        // for Rust to compile this code, we must handle all possible value patterns
        match coin {
            Coin::Penny => {
                // we can use scopes if we want to calculate something and it cannot fit in one line
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // if a patterns accepts a parameter, we can even use them inside of arms
                println!("State querter from {:?}", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama));

    // with match flow we can use previously mentioned Option<T> enum where T will be type of the variable inside of it.
    // this function takes a parameter with option enum and returns an option enum
    // if taken parameter has value inside of it it adds to it
    // and if not, it returns None
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

    let five = plus_one(Some(5));
    let six = plus_one(five);
    let none = plus_one(None);

    // except of enums (which have known number of value patterns) all other values take one default action.
    // this is for every other patterns that developer not handled or not important for the context of the code.
    // it is what `default` does in Javascript switch

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // this will match every non handled patterns
                                     // _ => rerol() // here we opt-out every other values since we used underscore
                                     // _ => () // here we opt-out every other values and we don't want to run any code for them
    }

    fn add_fancy_hat() {};
    fn remove_fancy_hat() {};
    fn move_player(num_spaces: u8) {}
}

fn option_enum() {
    // There is a builtin enum called `Option`
    // It is usefull for scenario in which a value could be something or nothing.
    // Example scenario is this: requesting first item from list. If list is empty it would give nothing but if it is not an empty list it would give first value inside of it.
    // Expressing this scenario with type system, compiler will understand that it may be nothing and would compile the code.
    // Since Rust does not have null as value, implementation of null-like is enum `Option<T>`.
    // Definition of `Option` enum in standard library is like this:
    // It is used so often that you can use `Some` and `None` without `Option::` prefix.
    enum Option<T> {
        None,
        Some(T),
    }

    let some_number = Some(5);
    let some_char = Some('e');
}

// Our example for this case will be IP adresses. They must be V4 or V6.
enum IpAddrKind {
    V4,
    V6,
}

fn enum_values() {
    // we use enums as variable like this. enum namespace followed by double colon and enum value.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // it is also possible to use enums as parameter type. this means that parameter must have a type that matches with variants of enum
    fn route(ip_kind: IpAddrKind) {
        // do something with ip_kind
    }

    // enums also can be used as type of struct field
    struct IpAddrStruct {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Representing same concept as before using just an enum is posible! We can put data directly into each enum variant.
    // Name of each enum variant that we define also becomes a function that constructs an instance of the enum.
    // Parameter types of enum variants don't need to be same at all. Here each variant have different number and type of parameters.
    // Parameter of enum variant can be any type, string, numeric values, structs and even another enums.
    enum IpAddrEnum {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // we can also define methods on enums just like structs.
    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move with x as {x} and y as {y}"),
                Message::Write(str) => println!("Write with {str}"),
                Message::ChangeColor(c1, c2, c3) => println!("Change Color with {c1}, {c2}, {c3}"),
            }
        }
    }

    let m = Message::Write(String::from("hello"));

    m.call();
}
