
pub fn entry() {
    let data = String::from("hey i like apples and banana");
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut result = String::new();

    for elem in data.split_whitespace() {
        let first = elem.chars().next().unwrap();
        if vowels.contains(&first) {
            result = result + &elem + "-hay";
        } else {
            result = format!("{}{}-{}ay", result, &elem[1..], &elem.chars().next().unwrap());
        }
        result = result + " ";
    }
    result.pop();
    println!("the string \"{}\" in pig latin gives \"{}\"",data , result)
}
