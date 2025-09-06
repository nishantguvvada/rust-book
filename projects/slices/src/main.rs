fn main() {
    // write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
    // If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content

    println!("word {word}");
    println!("string {s}");

    // Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //  we’ll convert our String to an array of bytes using the as_bytes method.

    for (i, &item) in bytes.iter().enumerate() {
        // we create an iterator over the array of bytes using the iter method
        if item == b' ' {
            return i;
        }
    }

    s.len()
} // returns index of the end of the first word in the string
  // the return value is only meaningful in context of the &String

fn sliced_first_word(s: &String) -> &str {
    // Now when we call sliced_first_word, we get back a single value that is tied to the underlying data
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
