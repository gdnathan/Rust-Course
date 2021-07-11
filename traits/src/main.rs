fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = 0;
    let mut i = 0;

    for &item in list {
        i += 1;
        if item > list[i] {
            largest = i;
        }
    }

    &list[largest]
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
