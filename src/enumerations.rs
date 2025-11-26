pub fn enumerations() {
    let msg_write = Message::Write(String::from("hello"));
    let msg_quit = Message::Quit;
    let msg_move = Message::Move { x: 30, y: 50 };
    let msg_change_color = Message::ChangeColor(0, 0, 0);

    msg_write.call();
    msg_quit.call();
    msg_move.call();
    msg_change_color.call();

    let dice_roll = 4;
    match dice_roll {
        1 => println!("1"),
        2 => println!("2"),
        other => println!("{other}"),
    }

    match dice_roll {
        3 => println!("3"),
        5 => println!("5"),
        _ => println!("{:?}", ()), // unit value
    }

    // Using 'if let' can be seen as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values
    // The syntax 'if let' takes an arm-pattern and an match-expression separated by an equal sign
    // It works the same way as a match, where the expression is given to the match and the pattern is its first arm
    //     - if let arm-pattern = match-expression {}
    //     - else {}
    //
    //     - match match-expression {
    //           arm-pattern => matched,
    //           _ => else,
    //    }

    if let 3 = dice_roll {
        println!("3");
    }

    if let 5 = dice_roll {
        println!("5");
    } else {
        println!("{:?}", ());
    }

    let msg = Message::Quit;
    if let Message::Quit = msg {
        println!("Quit");
    }
}

pub fn options() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_value: Option<i32> = None;

    let five = some_number;
    let six = option_plus_one(five);
    let none = option_plus_one(absent_value);
}

// This enum has four variants with different types
//     - Quit has no data associated with it at all
//     - Move has named fields like a struct does
//     - Write includes a single String
//     - ChangeColor includes three i32 values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Matches are exhaustive and their arms must cover all variants
        match self {
            Self::Quit => println!("Quit"),
            Self::Move { x, y } => println!("Move({x},{y})"),
            Self::Write(v) => println!("Write({v})"),
            Self::ChangeColor(r, g, b) => println!("ChangeColor({r},{g},{b})"),
        }
    }
}

fn option_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
