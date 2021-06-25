use std::collections::{BinaryHeap, HashMap};
use std::io;

pub fn entry() {
    let mut input = String::new();
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!(
            "---\nchoose a command:\n0: exit\n1:display info\n2: display all\nor enter \"Add [name] to [department]\"!\n---"
        );
        io::stdin().read_line(&mut input).expect("uh...");
        match input.trim() {
            "0" => break,
            "1" => display_dep(&data),
            "2" => display_all(&data),
            _ => add_element(&mut data, &input),
        }
        input.clear();
    }
}

fn add_element(data: &mut HashMap<String, Vec<String>>, elem: &String) {
    let parsed: Vec<&str> = elem.split_whitespace().collect();
    if parsed.len() != 4 || parsed[0] != "Add" || parsed[2] != "to" {
        println!("Invalid input");
        return;
    }
    data.entry(parsed[3].to_owned())
        .or_insert(vec![])
        .push(parsed[1].to_owned());
    // another way to do it

    // match data.get_mut(parsed[3]) {
    //     Some(n) => {
    //         n.push(String::from(parsed[1]));
    //     },
    //     None => {
    //         data.insert(String::from(parsed[3]), vec!(String::from(parsed[1])));
    //     },
    // };
}

fn display_dep(data: &HashMap<String, Vec<String>>) {
    let mut dep = String::new();
    println!("Choose a department");
    io::stdin().read_line(&mut dep).expect("bite");
    match data.get(dep.trim()) {
        Some(e) => {
            for elem in e {
                println!("{}", elem);
            }
        }
        None => println!("No such department."),
    }
}

fn display_all(data: &HashMap<String, Vec<String>>) {
    let mut tmp: Vec<_> = data.iter().collect();
    tmp.sort();
    for (dep, empls) in tmp {
        println!("{}:\n", dep);
        for emp in empls {
            println!("- {}", emp);
        }
    }
}
