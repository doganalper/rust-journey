use std::collections::HashMap;

// Collections
// In addition to array and tuple types, rust also has another data structure to contain multiple values inside of one variable. Collections are stored on the heap, this means size of collection is not fixed, it can be changed.
// Value of the collection does not need to be known at compile time.

// There's three collections in Rust:
// * A vector allows developers to store a variable number of values next to each other.
// * A string is a collection of chars.
// * A hash map allows developers to associate a value with a particular key.

fn main() {
    // vectors();
    // strings();
    hashmap();
}

fn hashmap() {
    // Hashmap `HashMap<K,V>` stores keys with `K` type to values of `V` type using `hashing function`. This function determines how it places these keys and values into memory.
    // Hash maps store data on the heap.
    // All types of keys and values must be same as other keys and values.

    // CREATING NEW HASH MAP
    // using `new` method
    let mut scores = HashMap::new();

    // INSERTING NEW KEY VALUE TO HASH MAP
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // accessing value from hash map using key
    // here get will return optional reference of value
    // copied will return value directly (no reference)
    // unwrap_or will handle options so if copied returns `Some` it will return value but
    // if copied returns None it will return given default value which is 0 here.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // we can iterate each key value pairs by referencing hash map with for loop
    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    // Types that implement `Copy` trait, they will be copied when used in hash maps.
    // But for owned values like `String`, ownership will be moved to hash map.
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Both field_name and field_value will not be accesible after this point since owner of there are now `map` variable.
    // This can be changed by passing reference for key or value. This way key and value variable will be usable as long as hash map is valid.

    // UPDATING HASH MAP
    // Overwriting existing value:
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Previous value of `10` will be overwritten here.

    println!("{:?}", scores); // will print {"Blue": 25}

    // Adding K/V if key does not exist
    // Hash maps have special API for this use case called `entry`. It takes the key that you want to check/insert and checks whether this key exists on hash map.
    // Return value of `entry` is an enum `Entry` that represents a value that might or might not exist.
    // `or_insert` method here will return mutable reference to existing value and if no value exists with given key it will insert given value with given key.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // This will not insert since value with key "Blue" exists.

    println!("{:?}", scores); // will print {"Blue": 25, "Yellow": 50}

    // Updating value based on the current (old) value
    // Sometimes we want to take actions based on the current value if it exists and if not insert it and continue to the action.
    // Example usage of this might be counting words from string. If we encounter first time with a word count will be 1 and next time it will be incremented by 1.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // we can change value of current key since `or_insert` returns a mutable reference and access current value by dereferencing
    }

    println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}

    // Default hashing function is called `SipHash`. It is not the fastest one but it has resistance to DOS attack. So by default Rust prefers better safety in sacrifice of speed here. But it can be changed in need.
}

fn strings() {
    // Strings (owned strings) in Rust are implemented as a collection of bytes.

    // CREATING A NEW STRING:
    // Many of the operations available with Vec<T> are also available with String as well, because String is implemented as a wrapper around a vector of bytes.
    // Creating new string instance
    let mut s = String::new();

    // Creating a new string instance from string literal
    let data = "initial contents".to_string();

    // Another way of creating string instance from string literal
    let s = String::from("initial contents");
    // Above two examples does not difference in performance and memory usage

    // Strings are UTF-8 encoded, so they can store any properly encoded data in them
    let hello = String::from("Здравствуйте");
    let hello = String::from("Dobrý den");

    // UPDATING STRING
    // Since it is a wrapper around of Vec<T>, Strings can grow in size and its contents can change.
    // To concat String value you can use `+` or `format!` macro

    // APPENDING TO STRING
    let mut s = String::from("foo");
    // `push_str` takes a reference to string slice because it does not need to take ownership of given string slice
    // and hence we can use value afterwards
    let s2 = "bar";
    s.push_str(s2);
    println!("s2: {s2}");

    // push method only accepts one char value
    let mut s = String::from("lo");
    s.push('l');

    // CONCATENATION WITH `+` OPERATOR OR THE FORMAT! MACRO
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // We can concat two string using `+` operator.
    // Note that value of s1 moved to s3 and s1 is invalid after here while s2 will be still valid.
    let s3 = s1 + &s2;
    // Under the hood plus operator signature is like this:
    // fn add(self, s: &str) -> String {}
    // Since we know compiler will act like &str and &String as same we can use &s2 which is &String.
    // Compiler will use deref coercion, turning &s2 into &s2[..]

    // Concating multiple strings is easier using `format!` macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-[s3]");

    // INDEXING INTO STRINGS
    // Rust does not support string indexing. Why?
    // A String is a wrapper over a Vec<u8>.
    let hello = String::from("Hola");
    // Here `len` will be 4, that means vector storing the string `Hola` is 4 bytes long.
    let hello_rus = String::from("Здравствуйте");
    // Here `len` will be 24 while there is 12 chars in that word. Because that is the number of bytes it takes to encode that word in UTF-8. Reason for that is each Unicode scalar value in that string takes 2 bytes of storage.
    // Because of this problem indexing owned string will not always correlate to valid Unicode scalar value.
    // ;If we would try indexing string it would return the byte value in that index not the char itself.

    // If we really want to get slice of string we could use range syntax but it is not very safe to use.
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // here `s` will be `Зд` because we know that those characters was each 2 bytes so we get with range 0 to 3.

    // ITERATING OVER STRINGS
    // Best way to operate on pieces of strings is to be explicit about you want chars or bytes. For unicode scalar values, use the `chars` method to get each chars inside of string.
    for c in "Зд".chars() {
        println!("{c}");
    }

    // For using bytes, `bytes` method returns each raw byte
    for b in "Зд".bytes() {
        println!("{b}");

        if b == b'3' {
            println!("GG")
        }
    }
}

fn vectors() {
    // Vectors have `Vec<T>` type where T is the type of elements it will store.
    // Values inside of vector are stored next to each other in memory.

    // Creating new vector
    // Since no values are added on initialization, we must add type annotation here
    // to give Rust a hint about what type it will store.
    // v will store multiple i32 type variable
    let v: Vec<i32> = Vec::new();

    // Most often vectors will be created with initial values so no need to annotate type
    // on this situation.
    // Using `vec!` macro, creates a new vector that holds the values given to it.
    let v = vec![1, 2, 3];

    // By default vectors are immutable and new values cannot be pushed or popped.
    // This can be changed with `mut` keyword.
    // Here compiler will infer the type this vector will store, thanks to type of variable being pushed on later lines.
    let mut v = Vec::new();

    v.push(5);
    v.push(6);

    // Reading elements from vectors
    // There is two ways to read elements, one is by indexing or using `get` method.
    // Both ways will give reference of the element.

    // This way is less secure and program will panic if given index is out of range
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is: {third}");

    // using `get` method will return an Option enum since there maybe
    // no item on the given index
    let third = v.get(2);

    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There is no third element."),
    }

    // Iterating over the values in a vector
    let v = vec![100, 32, 27];

    // We can iterate through reference of vector variable to access each item in it.
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 27];

    for i in &mut v {
        // in order to mutate item inside of a mutable vector, we must dereference
        // using (*) to get to the value in i.
        *i += 50
    }
}
