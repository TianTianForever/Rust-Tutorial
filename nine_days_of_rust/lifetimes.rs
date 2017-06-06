// fn take_word(s: &String) -> &str { }  
//it can only use 'String' type.

// This function allow us to use 'String' and '&str' types.
pub fn take_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item ==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// This function 'first_word' have a BUG !!!
pub fn first_word<'a>(s: &'a String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
