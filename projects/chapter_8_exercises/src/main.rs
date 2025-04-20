use core::fmt;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum MessageType {
    AddEmployee(String, Department),
    RemoveEmployee(String, Department),
    InvalidMessage,
    Exit,
}

#[derive(Debug, Hash, Eq, PartialEq)] // explicitly derive Hash for it to be usable as a key.
enum Department {
    Engineering,
    Sales,
    Accounting,
}

impl FromStr for Department {
    type Err = (); //  unit type, often used for “no information” errors.
    fn from_str(input: &str) -> Result<Department, Self::Err> {
        match input {
            "Engineering" | "eng" => Ok(Department::Engineering),
            "Sales" | "sal" => Ok(Department::Sales),
            "Accounting" | "acc" => Ok(Department::Accounting),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Department::Engineering => "Engineering",
            Department::Accounting => "Accounting",
            Department::Sales => "Sales",
        };
        write!(f, "{s}")
    }
}

fn main() {
    // 1. Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode
    // (the value that occurs most often; a hash map will be helpful here) of the list.

    let list = [420, 1, 2, 3, 4, 50, 69, 420];
    println!("list is {:?}", list);

    let median = get_median(list);
    println!("Median from list is {median}");

    let mode = get_mode(list);
    println!("Mode from list is {mode}");

    println!("");

    // 2. Convert strings to pig latin. The first consonant of each word is moved to the end of the
    // word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added
    // to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

    let test_string = "I like anime".to_string(); // I-hay ike-lay anime-hay
    let pig_correct = "I-hay ike-lay anime-hay".to_string();
    let my_attempt = convert_to_pig_latin(&test_string);
    println!("Pig attempt: {my_attempt}");
    if pig_correct == my_attempt {
        println!("Pig Correct !");
    } else {
        println!("Pig NOT correct :(");
    }

    // 3. Using a hash map and vectors, create a text interface to allow a user to add employee names
    // to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company
    // by department, sorted alphabetically.

    let mut record_keeper: HashMap<Department, Vec<String>> = HashMap::new();
    record_keeper.insert(Department::Engineering, vec![]);
    record_keeper.insert(Department::Sales, vec![]);
    record_keeper.insert(Department::Accounting, vec![]);

    println!("");
    println!("Add or remove employees");

    loop {
        let user_input = get_user_input();
        let message = extract_message_type(&user_input);
        match message {
            MessageType::AddEmployee(name, dep) => {
                println!("Adding! Name is {name}. Department is {dep}");
                record_keeper.entry(dep).or_insert_with(Vec::new).push(name);
            }
            MessageType::RemoveEmployee(name, dep) => {
                println!("Removing! Name is {name}. Department is {dep}");
                if let Some(vec) = record_keeper.get_mut(&dep) {
                    vec.retain(|x| x != &name);
                }
            }
            MessageType::Exit => {
                println!("Exiting ...");
                break;
            }
            MessageType::InvalidMessage => println!("Invalid message, type again ..."),
        }

        for (key, val) in &record_keeper {
            println!("{key}, {:?}", val);
        }
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read user input");

    user_input
}

// Exit
// Add <Name> to <Department>
// Remove <Name> from <Department>
fn extract_message_type(message: &str) -> MessageType {
    let words = message.split_whitespace().collect::<Vec<&str>>();
    if words[0] == "Exit" {
        return MessageType::Exit;
    }
    if words.len() < 4 {
        return MessageType::InvalidMessage;
    }

    if words[0] == "Add" || words[0] == "add" {
        if let Ok(department) = words[3].parse::<Department>() {
            return MessageType::AddEmployee(words[1].to_string(), department);
        } else {
            return MessageType::InvalidMessage;
        }
    }

    if words[0] == "Remove" || words[0] == "remove" {
        if let Ok(department) = words[3].parse::<Department>() {
            return MessageType::RemoveEmployee(words[1].to_string(), department);
        } else {
            return MessageType::InvalidMessage;
        }
    }

    MessageType::InvalidMessage
}

fn convert_to_pig_latin(str: &str) -> String {
    let mut result_vector = vec![];
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    for word in str.split_whitespace() {
        for c in word.chars() {
            if vowels.contains(&c) {
                result_vector.push(format!("{word}-hay"));
                break;
            } else {
                let skip_first = word.chars().skip(1).collect::<String>();
                result_vector.push(format!("{skip_first}-{c}ay"));
                break;
            }
        }
    }

    result_vector.join(" ").to_string()
}

fn get_median(arr: [i32; 8]) -> i32 {
    let mut v = arr.to_vec();
    v.sort();

    let mid_idx = v.len() / 2; // integer division leads to integer
    let mid_exist = v.get(mid_idx);

    match mid_exist {
        Some(num) => *num,
        None => 0,
    }
}

fn get_mode(arr: [i32; 8]) -> i32 {
    let mut ans = arr[0];
    let mut counter = HashMap::new();

    for i in arr.iter() {
        let count = counter.entry(i).or_insert(0);
        *count += 1;
    }

    for (key, value) in &counter {
        if value > &ans {
            ans = **key
        }
    }

    return ans;
}
