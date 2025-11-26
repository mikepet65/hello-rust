pub fn ownership() {
    // Each value in Rust has an owner
    // There can only be one owner at a time
    // When the owner goes out of scope, the value will be dropped
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait

    {
        // s is of type &str which is a string slice that points into the binary
        let s = "Hello, world!";
    } // s goes out of scope and is no longer valid

    {
        // s is of type String, stored on the stack while its text content is stored on the heap
        let mut s = String::from("Hello");
        s.push_str(", World!");
    } // s goes out of scope, it no longer valid, and its text context is dropped from the heap

    {
        // Everything known at compile-time, both variables are on the stack, y is a copy of x
        // i32 type implements Copy trait
        let x = 5;
        let y = x;

        // String does not implement the Copy trait and will be moved
        let s1 = String::from("Hello!");
        let s2 = s1;

        // Create a deep copy of a String (stack and heap data)
        let s3 = s2.clone();

        // println!("{s1}"); // Compiler error: borrow of moved value
        println!("{s2}, {s3}");
    } // Text contents of s2 and s3 are dropped from the heap

    {
        let s = String::from("Hello, World!");
        takes_ownership(s);
        // println!("{s}"); // Compiler error: borrow of moved value

        let mut s = gives_ownership();
        s.push_str(", Rust!");
        let l = borrowing(&s); // Take reference of s to borrow it
        println!("{l}, {s}");

        let mut s = gives_ownership();
        mutable_borrowing(&mut s); // Only one mutable reference at a time to avoid data races
        println!("{s}");
    }

    {
        let mut s = String::from("Hello, World!");

        let r1 = &s; // Immutable reference scope starts here
        let r2 = &s; // Second immutable reference
        println!("{r1} and {r2}"); // Immutable reference scope ends here

        let r3 = &mut s; // Mutable reference scope start here
        println!("{r3}"); // Mutable reference scope ends here

        // println!("{r1} and {r2}"); // Compiler error above: cannot borrow `_s` as mutable because it is also borrowed as immutable
    }

    string_slices();
    int_slices();
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn gives_ownership() -> String {
    let mut some_string = String::new();
    some_string.push_str("Hello");
    return some_string;
}

fn borrowing(some_string: &String) -> usize {
    some_string.len()
}

fn mutable_borrowing(some_string: &mut String) {
    some_string.push_str(", World!");
}

fn string_slices() {
    let s = String::from("Hello World");
    let hello = &s[0..5]; // Reference to a part of a String 'hello'
    let hello = &s[..5]; // Reference to a part of a String 'hello'
    let world = &s[6..11]; // Reference to a part of a String 'world'
    let world = &s[6..]; // Reference to a part of a String 'world'
    println!("{hello}, {world}!");

    let len = s.len();
    let hello_world = &s[0..len];
    let hello_world = &s[..];
    println!("{hello_world}!");
}

fn int_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    let b = [2, 3];
    assert_eq!(slice, &b);
    println!("{:?}", slice == &b && slice == &[2, 3] && slice == b);
}
