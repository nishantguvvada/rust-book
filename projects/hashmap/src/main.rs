// use std::collections::HashMap;

fn main() {
    // let mut v = Vec::new();

    // v.push(4);
    // v.push(9);
    // v.push(2);
    // v.push(7);
    // v.push(3);
    // v.push(8);
    // v.push(1);

    // println!("{v:?}")
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    // let mut v = vec![4, 4, 4, 5, 9, 2, 1, 1, 6, 7, 3, 3, 8, 1];
    // println!("Original vector: {v:?}");
    // v.sort();
    // println!("Sorted vector: {v:?}");

    // let n = v.len();
    // let middle = n / 2;
    // println!("Middle index: {middle}");
    // let median_value = v.get(middle);
    // match median_value {
    //     Some(value) => println!("The median value is: {value}"),
    //     None => println!("NONE"),
    // };

    // let mut map = HashMap::new();

    // for i in &v {
    //     let count = map.entry(i).or_insert(0);
    //     *count += 1
    // }
    // println!("Map: {map:?}");

    // let mut max_key = &&0;
    // let mut max_value = 0;

    // for (k, v) in &map {
    //     if *v > max_value {
    //         max_key = k;
    //         max_value = *v;
    //     }
    // }

    // println!("Most frequent element in the list is: {max_key}:{max_value}");

    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
    // so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
    // Keep in mind the details about UTF-8 encoding!

    println!("{}", pig_latin_word("first")); // irst-fay
    println!("{}", pig_latin_word("apple")); // apple-hay
    println!("{}", pig_latin_word("Hello")); // ello-Hay
}

fn pig_latin_word(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let chars: Vec<char> = word.chars().collect();

    if chars.is_empty() {
        return String::new();
    }

    let first = chars[0];
    let first_lower = first.to_ascii_lowercase();

    if vowels.contains(&first_lower) {
        let mut s = String::with_capacity(word.len() + 4);
        s.push_str(word);
        s.push_str("-hay");
        s
    } else {
        let mut s = String::new();
        for &c in &chars[1..] {
            s.push(c);
        }
        s.push_str("-");
        s.push(first);
        s.push_str("ay");
        s
    }

    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
    // for example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
}

// EXPLANATION OF PIG-LATIN FUNCTION
// ---

// ## 🧩 1. What does `chars()` do?

// When you call:

// ```rust
// let chars_iter = word.chars();
// ```

// It creates an **iterator** over the *Unicode scalar values* (i.e., `char`s) in the string.

// Rust `String` is UTF-8 encoded internally — which means a single visible character (like `é`) can take **more than one byte**.
// `chars()` safely decodes those bytes into individual `char` values.

// Example:

// ```rust
// let word = "café";
// for c in word.chars() {
//     println!("{}", c);
// }
// ```

// Output:

// ```
// c
// a
// f
// é
// ```

// Each `c` here is a `char` (4 bytes in memory, a Unicode scalar).
// You cannot safely do something like `word[0]` in Rust because `String` is byte-based, not char-based — slicing at byte boundaries could break UTF-8 encoding.

// That’s why we call `.chars()` when we want to *work with characters*.

// ---

// ## 📦 2. What does `.collect()` do?

// `collect()` **consumes an iterator** and collects all its items into a collection — such as a `Vec`, `String`, or `HashSet` — depending on what type you specify or what the compiler infers.

// Example:

// ```rust
// let chars: Vec<char> = "hello".chars().collect();
// println!("{:?}", chars);
// ```

// Output:

// ```
// ['h', 'e', 'l', 'l', 'o']
// ```

// So after `.collect()`, you no longer have an iterator; you have a `Vec<char>` containing all the individual characters.

// ---

// ## 🧱 3. Why are we creating a `Vec<char>` in the first place?

// We create `Vec<char>` because we need to **access characters by index** (like `chars[0]`, `chars[1..]`, etc.).

// Example:

// ```rust
// let mut chars: Vec<char> = "first".chars().collect();
// let first = chars[0];         // okay
// let rest = &chars[1..];       // okay
// ```

// If we only had the iterator (from `.chars()`), we **can’t** do indexing.
// Iterators in Rust are **one-way streams** — you can only call `.next()` to get the next value.
// You can’t go backwards, peek multiple times, or slice them.

// So:

// * If you only need to *consume* characters one by one (like reading letters sequentially), `chars()` alone is fine.
// * But if you want random access or slicing, you first `.collect()` into a `Vec<char>`.

// That’s why in Pig Latin, since we need to:

// * access the **first letter**, and
// * access the **remaining letters**,
//   we use a `Vec<char>`.

// ---

// ## 🧮 4. Why check if `chars` is empty?

// ```rust
// if chars.is_empty() {
//     return String::new();
// }
// ```

// This handles an **edge case** — what if the function is called with an empty string, `""`?

// Example:

// ```rust
// println!("{}", pig_latin_word(""));
// ```

// Without that check, trying to access `chars[0]` would **panic** (`index out of bounds`).

// Even if your logic assumes “there will always be an input word”, this is **defensive programming**:

// * Functions should gracefully handle empty or malformed input.
// * It’s better to explicitly handle it once than risk a runtime panic later.

// So it’s just good practice and shows robustness.

// ---

// ## ⚙️ 5. What does `with_capacity()` do?

// `String::with_capacity(n)` creates an **empty string** but *pre-allocates memory* for `n` bytes.

// Example:

// ```rust
// let mut s = String::with_capacity(10);
// ```

// Now `s` can grow up to 10 bytes before needing to reallocate.

// This is a **performance optimization** — it avoids multiple heap reallocations as we append text.

// Why 10 bytes here? We’re just estimating:
// if the word has, say, 5 characters, and we’re adding “-hay” (4 more), 9 bytes total fits comfortably.

// So:

// ```rust
// let mut s = String::with_capacity(word.len() + 4);
// ```

// prepares enough memory for the final string, since we know we’ll append `"-hay"`.

// It’s **not required**, but it avoids small memory reallocations and is a sign of clean, optimized code.

// ---

// ## ⚖️ 6. Why not use `with_capacity()` in the `else` branch?

// Good catch — technically we **could**, but it’s **optional** because:

// * In the consonant branch, we build `s` manually by pushing characters one by one.
// * The number of characters is roughly `word.len() + 3` (`-` + `ay`).
// * For short words, the difference in performance is *negligible*.
// * The vowel branch is often written with `with_capacity()` because we directly append the full original `word` string, which might be longer.

// If we wanted perfect consistency, we could write:

// ```rust
// let mut s = String::with_capacity(word.len() + 3);
// ```

// in the `else` branch too — it’s just not *needed* unless you’re handling large numbers of long strings.

// ---

// ## 🧠 Quick summary

// | Concept                             | What it does                                              | Why we use it here                                           |
// | ----------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------ |
// | `chars()`                           | Returns iterator over Unicode `char`s                     | Safely iterate through characters in a UTF-8 `String`        |
// | `collect()`                         | Consumes iterator and collects into `Vec`, `String`, etc. | We collect into `Vec<char>` to access by index               |
// | `Vec<char>`                         | Vector of characters                                      | Enables slicing like `[1..]` and getting first letter easily |
// | `is_empty()` check                  | Prevents panic on empty input                             | Defensive programming                                        |
// | `with_capacity(n)`                  | Preallocates memory for string                            | Improves performance by avoiding reallocations               |
// | Not using `with_capacity()` in else | Optional; minor performance difference                    | Simplicity over micro-optimization                           |

// ---

// ## Example illustrating all together

// ```rust
// fn pig_latin_word(word: &str) -> String {
//     let vowels = ['a', 'e', 'i', 'o', 'u'];
//     let chars: Vec<char> = word.chars().collect();

//     if chars.is_empty() {
//         return String::new(); // avoid panic
//     }

//     let first = chars[0];
//     let first_lower = first.to_ascii_lowercase();

//     if vowels.contains(&first_lower) {
//         // Pre-allocate to optimize
//         let mut s = String::with_capacity(word.len() + 4);
//         s.push_str(word);
//         s.push_str("-hay");
//         s
//     } else {
//         let mut s = String::new(); // could also use with_capacity(word.len() + 3)
//         for &c in &chars[1..] {
//             s.push(c);
//         }
//         s.push('-');
//         s.push(first);
//         s.push_str("ay");
//         s
//     }
// }
// ```

// ---
