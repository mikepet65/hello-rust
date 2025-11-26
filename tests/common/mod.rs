// Files in subdirectories of the tests directory don’t get compiled as separate crates

pub fn setup() {
    // Setup code specific to your library's tests would go here
    println!("common::setup");
}

pub fn teardown() {
    // Teardown code specific to your library's tests would go here
    println!("common::teardown");
}
