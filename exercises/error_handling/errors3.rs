// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

// DONE

use std::num::ParseIntError;

// need to change return type..
fn main() -> Result<(),ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    /* 
    // Method 1
    if cost > tokens {
        Ok(println!("You can't afford that many!"))
    } else {
        tokens -= cost;
        Ok(println!("You now have {} tokens.", tokens))
    }
    */

    // Method 2 : I feel weird passing println! into Ok()
    if cost > tokens {
        println!("You can't afford that many!")
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens)
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
