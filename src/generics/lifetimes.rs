// Lifetime annotations don’t change how long any of the references live. 
// Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. 
// Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

// When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. 
// In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

// When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.

// The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
// These rules are called 'lifetime elision rules'.
// The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
// The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

// Generic lifetime parameter, not a generic type
pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This lifetime parameter means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

// Generic lifetime parameter, not a generic type
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

// The lifetime parameter declaration after impl and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
