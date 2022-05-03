// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// DONE

fn main() {
    //let x = String::from("10");
    let x: u8 = 0;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}

/*
The compiler message is saying that Rust cannot infer the type that the
variable binding `x` has with what is given here.
What happens if you annotate line 7 with a type annotation? 
if the type is sufficient space(e.g. u32), but uninitialized, throw uninitialized vaariable x error.
if it's initialized is ok.
What if you give x a value? integer value is okay, else error at if statement
What if you do both? is okay as long as it's integer.
What type should x be, anyway? integer (for this example, unsigned 8 bit is ok)
What if x is the same type as 10? What if it's a different type? the if condition will fail, as cannot compare string to integer.
*/
