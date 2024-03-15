pub mod cleaner;

// this function is private to the bathroom module BUT can still be called
// from the cleaner module. This is because the cleaner module is a child of
// the bathroom module
fn clean() {
    println!("cleaning bathroom");
}
