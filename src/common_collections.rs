// Unlike the built-in array and tuple compound types, the data these collections point to is stored on the heap.
pub fn collections() {
    vectors();
    strings();
    hash_maps();
}

// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory
fn vectors() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new(); // Type inference through `push` methods afterwards
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third = &v[2];
    println!("Third element is {third}");

    let third = v.get(2);
    match third {
        Some(value) => println!("Third element is {value}"),
        None => println!("There is no third element"),
    }

    // LIFO: 5, 4, 3, 2, 1
    while !v.is_empty() {
        println!("{}", v.pop().unwrap());
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![7, 8, 9];
    for i in &mut v {
        *i += 10;
        println!("{i}");
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Use enum to store values of different types in one vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.14),
    ];

    for c in &row {
        println!("{:?}", c);
    }
}

// Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text
// String slices of type `str` are references to some UTF-8 encoded string data stored elsewhere
// Rust has only one string type in the core language, which is the string slice `str`
fn strings() {
    // Strings are UTF-8 encoded
    let mut s = String::new();
    s.push('H');
    s.push('e');
    s.push('l');
    s.push('l');
    s.push('o');
    s.clear();
    s.push_str("Hello");

    // The `to_string` method is available on any type that implements the `Display` trait
    let s = "Hello".to_string();
    let s = String::from("Hello");

    // The + operator uses the associated add method: fn add(self, s: &str) -> String;
    // the `hello` variable is moved into the add method and will no longer be valid after that
    // Compiler can coerce the &String argument into a &str
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world;
    println!("{hello_world}");

    // The format! macro uses references so that this call doesn’t take ownership of any of its parameters
    let one = String::from("one");
    let two = String::from("two");
    let three = String::from("three");
    let one_two_three = format!("{one}-{two}-{three}");
    println!("{one_two_three}");

    // A String is a wrapper over a Vec<u8>
    let hello = String::from("Hola"); // Each of these letters takes 1 byte when encoded in UTF-8, len == 4
    let hello = String::from("Здравствуйте"); // Each of these letters takes 1 byte when encoded in UTF-8, len == 24

    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes
    let hello = "Здравствуйте";
    let answer = hello.bytes().nth(0).unwrap();
    println!("Byte {answer}");
    let answer = hello.chars().nth(0).unwrap();
    println!("UTF-8 Char {answer}");

    for b in hello.bytes() {
        print!("{b} ");
    }
    println!();

    for c in hello.chars() {
        print!("{c} ");
    }
    println!();
}

// The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory
fn hash_maps() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    // This HashMap has keys of type String and values of type i32
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // Accessing values in a hash map
    let team_name = String::from("blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team score = {team_score}");

    // Borrow &k and &v from &scores
    for (k, v) in &scores {
        println!("{k} = {v}");
    }

    // Overwriting values in a hash map
    scores.insert(String::from("green"), 10);
    scores.insert(String::from("green"), 25);
    println!("{scores:?}");

    // Adding a key and value only if a key isn’t present
    scores.entry(String::from("green")).or_insert(75);
    scores.entry(String::from("orange")).or_insert(35);
    println!("{scores:?}");

    // Updating a value based on the old value
    let some_text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in some_text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{word_count:?}");
}
