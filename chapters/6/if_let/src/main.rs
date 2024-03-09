fn main() {
    set_max(Some(3));
    refactored_set_max(Some(5));
}

// we have to handle all cases which leads to unnecessary boilerplate code _ => ()
fn set_max(max: Option<u8>) {
    match max {
        Some(max) => {
            println!("max: {}", max);
        }
        _ => (),
    }
}

// here we can use if let to handle only the case we are interested in
fn refactored_set_max(max: Option<u8>) {
    if let Some(max) = max {
        println!("max: {}", max);
    }
}
