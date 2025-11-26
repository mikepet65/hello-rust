#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl
// Multiple impl blocks are allowed
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    fn area2(self: &Self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function without self parameter, called using the :: syntax
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
