mod user;
mod rectangle;

use crate::structs::{rectangle::Rectangle, user::*};

pub fn structs() {
    // Reminder: assignment means moving values if they don't have the Copy trait
    let mut user1 = User {
        email: String::from("someone1@example.com"),
        username: String::from("username1"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("someone_else1@example.com");

    // Struct update syntax where .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance
    let user2 = User {
        email: String::from("someone2@example.com"),
        ..user1
    };

    // println!("{:?}", user1); // Compiler error: borrow of partially moved value: `user1`
    println!("{:?}", user2);

    let user3 = build_user(
        String::from("someone3@example.com"),
        String::from("username3"),
    );

    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // The dbg! macro takes ownership of an expression and returns ownership of the value
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // Borrowing because of reference
    dbg!(&rect1);

    // When you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method
    // Method syntax is just syntactical sugar to call a associated function of a struct
    // Method calls use a Rust feature named: automatic referencing and dereferencing to avoid (&object).method() syntax
    println!("Area of rectangle = {}", rect1.area());
    println!("Area of rectangle = {}", (&rect1).area());
    println!("Area of rectangle = {}", Rectangle::area(&rect1));
    println!("Square rectangle = {:?}", Rectangle::square(30));
    println!("Square rectangle = {:#?}", Rectangle::square(30));
}

// Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
