// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// DONE

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
/*
In Rust, variable bindings are immutable by default. But here we're trying
to reassign a different value to x! There's a keyword we can use to make
a variable binding mutable instead.
 */