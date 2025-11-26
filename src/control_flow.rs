pub fn control_flow() {
    let n = 6;

    // An if expression allows you to branch your code depending on conditions
    // We place the block of code to execute if the condition is true immediately after the condition inside curly brackets
    // Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions
    if n % 4 == 0 {
        println!("Number is divisible by 4");
    } else if n % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is not divisible by 4 or 3");
    }

    // If is an expression and can be used on the right side of the let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop
    loop {
        break;
    }

    // Loop can be used on the right side of the let statement to return values
    let mut retry = 0;

    let result = loop {
        retry += 1;

        if retry == 3 {
            break retry;
        }
    };

    println!("Result: {retry}");

    // Loop labels to disambiguate between multiple loops
    let mut count = 0;

    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 5;

        loop {
            println!("Remaining = {remaining}");

            if remaining == 4 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // While the condition is true, the while loop runs
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    // Use a for to execute code for each item in a collection
    let a = [1, 2, 3];

    for e in a {
        println!("The value is: {e}");
    }

    for n in 1..=3 {
        println!("Number: {n}");
    }

    for c in (1..=3).rev() {
        println!("Countdown: {c}");
    }
}
