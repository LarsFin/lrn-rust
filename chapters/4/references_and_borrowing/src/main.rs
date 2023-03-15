fn main() {
    let s1 = String::from("hello");
    let len = len(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    append_world(&mut s2);
    println!("{}", s2);

    let mut s3 = String::from("hello");
    let r1 = &mut s3;
    // let r2 = &mut s3; // error[E0499]: cannot borrow `s3` as mutable more than once at a time
    println!("{}", r1);
    println!("{}", s3);
    // cannot borrow mutable values more than once, this is for rust to help prevent data races,
    // this can happen when these three conditions are met:
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There’s no mechanism being used to synchronize access to the data.

    // we can use braces to create scope and avoid multiple mutable borrows
    {
        let r3 = &mut s3;
        println!("{}", r3);
    }

    let r4 = &mut s3;
    println!("{}", r4);

    let r5 = &s3;
    let r6 = &s3;
    // let r7 = &mut s3; // error[E0502]: cannot borrow `s3` as mutable because it is also borrowed as immutable
    println!("{} and {}", r5, r6);

    // however, we can now borrow as mutable after the immutable borrows ones are dropped
    let r7 = &mut s3;
    append_world(r7);
    println!("{}", r7);
}

fn len(s: &str) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

// Rust ensures that references will never be dangling references
// fn dangle() -> &String { // returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // s goes out of scope here, and is dropped. Its memory goes away.
