struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

struct Position(f32, f32, f32);

// Unit structs can be used to share a trait across instances of a type - more on traits later...
struct UnitStruct;

// structs can also reference memory with different owners using references, it does however also
// require a lifetime specifier to ensure the reference doesn't outlive the owner, more in chpt 10
// struct Order {
//     number: String,
//     user: &User, // E0106 missing lifetime specifier
// }

fn main() {
    let u1 = User {
        active: true,
        name: String::from("John Doe"),
        email: String::from("john@test.com"),
        sign_in_count: 1,
    };

    // u1.name = false; // This will not work because u1 is immutable by default

    print_user_info(&u1);

    // the whole struct must be declared as mutable to change one field
    let mut u2 = build_user(String::from("Jaen Doe"), String::from("jane@test.com"));

    u2.name = String::from("Jane Smith");

    print_user_info(&u2);

    // it's possible to spread a struct into another struct too, but the syntax is a little
    // different to that of JS '..' vs '...'
    let u3 = User {
        email: String::from("jane.d@test.com"),
        ..u2
    };

    print_user_info(&u3);

    // print_user_info(&u2); // E0382 because 'name' was moved to u3 so u2 can no longer be used

    // if we only spread the COPY trait values, we can still use the original struct
    let u4 = User {
        name: String::from("Tony Barret"),
        email: String::from("tony@test.com"),
        ..u1
    };

    print_user_info(&u4);
    print_user_info(&u1);

    // we can also still use some of u2's fields, just none which were moved to u3 (and not copied)
    print!("Is user active? {}\n", u2.active);

    // we can't though pass the user to a function that takes ownership of the user, even if it doesn't
    // access the fields that were moved to u3
    // is_user_active(u2); // E0382
    is_user_active(u1);

    // tuple structs are effectively tuples which have a name
    let p1 = Position(0.0, 0.0, 5.0);
    print_position(p1);

    let _ = UnitStruct;
}

fn print_user_info(u: &User) {
    println!(
        "User name: {}\nActive?: {}\nUser email: {}\nUser sign in count: {}",
        u.name,
        u.active,
        u.email,
        u.sign_in_count
    );
}

fn is_user_active(u: User) -> bool {
    u.active
}

fn build_user(name: String, email: String) -> User {
    // same as javascript you can use the shorthand for key values on a struct
    User {
        active: true,
        name,
        email,
        sign_in_count: 1,
    }
}

fn print_position(p: Position) {
    // accessed by dot notation with the index of the tuple
    println!("Position: ({}, {}, {})", p.0, p.1, p.2);
}
