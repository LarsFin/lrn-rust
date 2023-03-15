fn main() {
    let mut s = String::from("Hello,");
    s.push_str(" world!");
    println!("{}", s);

    // let mut s1 = String::from("A1");
    // let s2 = s1;
    // s1.push_str("B2");
    // error[E0382]: use of moved value: `s1`
    // A value can only have one owner at a time. Here, s1's value is moved to s2 and
    // s1 is out of scope so no longer valid.
    // You'd have to clone or with self defined types derive the Copy trait.

    let mut x = 5;
    let y = x;
    x = 6;
    println!("x: {}, y: {}", x, y);
    // This works because x and y are integers which are stored on the stack and have a
    // known data size at compile time. So, the value is copied instead of moved.

    let mut s1 = String::from("Hello");
    append_world(&mut s1); // function borrows the mutable reference to s1
    println!("{}", s1);

    let s2 = String::from("Hello");
    take_ownership(s2); // function takes ownership of s2 and s2 is no longer valid
    // println!("{}", s2); // error[E0382]: borrow of moved value: `s2`

    let s3 = gives_ownership(); // function returns ownership of a new string
    println!("{}", s3);

    let mut s4 = gives_ownership();
    s4.push_str(" and mutated!");
    println!("{}", s4);

    let s5 = String::from("Hello");
    let s6 = takes_and_gives_back(s5); // function takes ownership of s5 and returns ownership of a new string
    // println!("{}", s5); // error[E0382]: borrow of moved value: `s5`
    println!("{}", s6);
}

fn append_world(s: &mut String) {
    s.push_str(" world!");
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn gives_ownership() -> String {
    String::from("I have been given")
}

fn takes_and_gives_back(s: String) -> String {
    s
}
