// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// DONE

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    
    // ***actual value in an element of the vector is an optional value, Option<i8>
    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let

    // while let interates through the vector, until it returns the None value
    // each element in vector is a Option<i8>, in which previous for loop pushes Some(...)

    // ***Note: optional_integers_vec is a vector, and .pop() returns an Option<T> type
    // T in this case is Option<i8>
    // so is actually returning Option<Option<i8>> type
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {}", integer);
    }
}
