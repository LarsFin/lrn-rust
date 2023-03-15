fn main() {
    let s = String::from("First word of a sentence!");

    let i = first_word(&s);
    println!("The first word is '{}'", &s[..i]);

    let w = first_word_slice(&s);
    println!("The first word is '{}'", w);

    let w = nth_word(&s, 2);
    println!("The second word is '{}'", w);

    let w = nth_word(&s, 10);
    println!("The tenth word is '{}'", w);
}

// first word implementation without slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// first word implementation with slices
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn nth_word(s: &str, n: u32) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if n == 1 {
                return &s[..i];
            } else {
                return nth_word(&s[i + 1..], n - 1);
            }
        }
    }

    return &"";
}