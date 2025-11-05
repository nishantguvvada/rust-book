use std::collections::HashMap;

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
    let mut v = vec![4, 4, 4, 5, 9, 2, 1, 1, 6, 7, 3, 3, 8, 1];
    println!("Original vector: {v:?}");
    v.sort();
    println!("Sorted vector: {v:?}");

    let n = v.len();
    let middle = n / 2;
    println!("Middle index: {middle}");
    let median_value = v.get(middle);
    match median_value {
        Some(value) => println!("The median value is: {value}"),
        None => println!("NONE"),
    };

    let mut map = HashMap::new();

    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1
    }
    println!("Map: {map:?}");

    let mut max_key = &&0;
    let mut max_value = 0;

    for (k, v) in &map {
        if *v > max_value {
            max_key = k;
            max_value = *v;
        }
    }

    println!("Most frequent element in the list is: {max_key}:{max_value}");
}
