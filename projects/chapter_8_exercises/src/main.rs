use std::collections::HashMap;

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
