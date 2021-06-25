
use std::collections::HashMap;

pub fn entry() {
    let list = [ 2, 4, 6, 5, 7, 9, 1, 3, 8, 4 ];
    println !("List: {:?}", list);
    let mut vec = list.to_vec();
    vec.sort();
    println!("ordered list: {:?}", vec);

    let mut average = 0;
    for elem in &vec {
        average += elem;
    }
    println!("Average: {}", average / vec.len());
    let median: &usize = &vec[vec.len() / 2];
    println!("Median: {}", median);
    let mut map = HashMap::new();

    for elem in list {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }
    let mut hightest_val = 0;
    let mut hightest_key = 0;
    for (key, val) in map {
        if val > hightest_val {
            hightest_val = val;
            hightest_key = key;
        }
    }
    println!("Mode: {}", hightest_key);
}
