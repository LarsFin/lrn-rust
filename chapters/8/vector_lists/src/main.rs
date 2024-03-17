fn main() {
    // creating a new empty vector of type i32 like all typed languages we need to
    // annotate the type when instancing an empty collection
    let _v: Vec<i32> = Vec::new();

    // the vector module also includes a macro to create a vector with initial values
    let mut v = vec![1, 2, 3];

    // similar to dynamic arrays in other languages we can use the push method to add
    // elements (provided the vector is mutable)
    v.push(4);
    v.push(5);

    // there are two ways to read an element from a vector; via indexing or with the
    // get method.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // the below commented out code does compile but the running code panics at runtime
    // let sixth = &v[5];
    // thread 'main' panicked at src/main.rs:
    // index out of bounds: the len is 5 but the index is 5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // however, we can use get as it'll return None if the index is out of bounds
    let sixth = v.get(5);
    match sixth {
        Some(sixth) => println!("The sixth element is {}", sixth),
        None => println!("There is no sixth element."),
    }

    let first = &v[0];

    // the line below wont compile due to how vectors work in rust, they store memory
    // based on their type and length, when extending the vector the memory is reallocated
    // and 'first' is no longer memory safe to use. If you comment line 44 it will compile
    // v.push(6);
    println!("The first element is: {}", first);

    let v = vec![100, 200, 300, 500];

    print_elements(&v);

    let mut v = vec![5, 7, 13];

    double_elements(&mut v);
    print_elements(&v);

    // we can use enums to store different types in a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    print_spreadsheet_row(row);
}

// as in other programming languages it's possible to iterate over the elements
pub fn print_elements(v: &Vec<i32>) {
    for i in v {
        println!("{}", i);
    }
}

// you can also iterate over mutable references to change the elements
pub fn double_elements(v: &mut Vec<i32>) {
    for i in v {
        // use a dereference (*) to access the value of the borrowed reference
        *i *= 2;
    }
}

pub fn print_spreadsheet_row(row: Vec<SpreadsheetCell>) {
    for i in row {
        match i {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}

pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
