fn main() {
    let this_string = "Será que vai funcionar?";
    let primeira_palavra = first_word(&this_string);
    // this_string.clear();
    println!("A primeira palavra é {primeira_palavra}")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
