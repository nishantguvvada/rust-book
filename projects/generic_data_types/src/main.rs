fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![1, 2, 4, 6, 2, 4];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'q', 't', 'u', 'e'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
