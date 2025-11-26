use std::thread;
use std::time::Duration;

// The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use.
// Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values.
//     - FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
//     - FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
//     - Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
pub fn closures() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1,
        store.giveaway(user_pref1)
    );

    let user_pref2 = None;
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2,
        store.giveaway(user_pref2)
    );

    // Parameter and return value types are annotated explicitely
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let r = expensive_closure(3);

    // Parameter and return value type inference happens at first invocation (types locked after that)
    let example_closure = |x| x;
    let s = example_closure("x");

    // Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter.

    // Borrowing immutably
    let borrowing_immutably = [3; 5];
    println!("Before defining closure: {:?}", borrowing_immutably);
    let borrow_immutably = || println!("From closure {:?}", borrowing_immutably);
    println!("Before calling closure: {:?}", borrowing_immutably);
    borrow_immutably();
    println!("After calling closure: {:?}", borrowing_immutably);

    // Borrowing mutably (scope set from first use to last use and not through scope {})
    let mut borrowing_mutably = vec![4; 6];
    println!("Before defining closure: {:?}", borrowing_mutably);
    let mut borrow_mutably = || borrowing_mutably.push(5);
    //println!("Before calling closure: {:?}", borrowing_mutably); // Compiler error: cannot borrow `borrowing_mutably` as immutable because it is also borrowed as mutable
    borrow_mutably();
    println!("After calling closure: {:?}", borrowing_mutably);

    // Taking ownership
    let taking_ownership = vec![1, 2, 3];
    println!("Before defining closure: {:?}", taking_ownership);
    thread::spawn(move || println!("From thread {:?}", taking_ownership))
        .join()
        .unwrap();
    // println!("After calling closure: {:?}", taking_ownership); // Compiler error: borrow of moved value: `taking_ownership`
}

// All iterators implement a trait named Iterator that is defined in the standard library
// Iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up
#[allow(unused_mut)]
pub fn iterators() {
    // Get immutable borrowing iterator from a vector
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // The for loop takes ownership of v1_iter and makes it mutable behind the scenes
    for val in v1_iter {
        println!("Got: {val}");
    }

    // Remember: this `v1_iter` shadows the first `v1_iter`
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // Get mutable borrowing and ownershop iterators from a vector
    let mut v2 = vec![4, 5, 6];
    let mut v2_iter_mut = v2.iter_mut();
    let mut v2_iter_own = v2.into_iter();

    // Consuming adaptors are methods defined on the Iterator trait that internally call next and use up the iterator
    let mut v1_iter = v1.iter();
    println!("Sum: {}", v1_iter.sum::<i32>()); // Consuming adaptor `sum`

    // Iterator adaptors are methods defined on the Iterator trait that produce different iterators by changing some aspect of the original iterator and don’t consume the iterator
    let mut v1_map = v1.iter().map(|x| x + 1); // Iterator adaptor `map`
    let v2: Vec<_> = v1_map.collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // For loop creates the iterator automatically
    for val in &v2 {
        println!("Got: {val}");
    }

    // Iterators with a closure that captures its environment
    const IMAX: i32 = 3;
    let v3: Vec<_> = v2.iter().filter(|i| i <= &&IMAX).collect(); // Iterator adaptor `filter`

    for val in &v3 {
        println!("Got: {val}");
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //user_preference.unwrap_or_else(|| -> ShirtColor {self.most_stocked()})
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
