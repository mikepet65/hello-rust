pub fn statements_and_expressions() {
    // Rust is an expression-based language
    //     - Statements are instructions that perform some action and do not return a value
    //     - Expressions evaluate to a resultant value
    //     - Calling a function is an expression
    //     - Calling a macro is an expression
    //     - A new scope block created with curly brackets is an expression

    let x = 1; // A statement with the expression 1
    let y = {}; // Expressions implicitly return the unit value () if they don’t return any other value

    let z = {
        let x = 1; // Statement
        x + 1 // Resultant value: expressions do not include ending semicolons
              // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
    };
}

pub fn five() -> i32 {
    5
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}
