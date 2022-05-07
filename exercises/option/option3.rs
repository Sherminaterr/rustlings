// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

// DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    /*
    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    */

    // the above does not work as Option<T> does not implement copy trait.
    // old value not usable

    // so we take y by reference.
    match &y {
        //Some(p) is an exact match, so borrow can be reliquished (not active)
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
