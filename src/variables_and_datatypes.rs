#[allow(unused_assignments)]
pub fn variables() {
    const XX: u32 = 1 * 2 * 3;

    let mut x = 5;
    let y: i32;
    x = 6;
    x += 2;

    let spaces = "   ";
    let spaces: usize = spaces.len();
}

pub fn data_types() {
    let i: u8 = 255;
    let j = u8::wrapping_add(i, 2u8);
    let o = u8::overflowing_add(i, 2u8);
    let c = u8::checked_add(i, 2u8);
    let s = u8::saturating_add(i, 2u8);
    println!("{j}, {s}, {:?}, {:?}", o, c);

    let x = 2.0;
    let y: f32 = 3.0;
    let z = 5.1f64;

    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);

    let t = true;
    let f: bool = false;

    // Four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z: char = 'Z';

    // Tuple compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (600, 7.4, 5);
    let (x, y, z) = tup;
    let six_hundred = tup.0;
    let five = tup.2;

    // Empty type and its empty value are called 'unit'
    // Expressions implicitly return the unit value if they don’t return any other value
    let a_unit: () = ();

    // Array compound type
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let third = a[3];
    println!("{}, {}", first, third);

    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{:?}", a)
}

pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
