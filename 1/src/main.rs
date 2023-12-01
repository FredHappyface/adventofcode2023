use std::collections::HashMap;
use std::fs;

fn replace_first(numbers: HashMap<&str, i32>, mut string: String) -> String {
    let mut idx = usize::MAX;
    let mut num = 0;
    for (key, val) in &numbers {
        let res = string.find(key).unwrap_or(usize::MAX);
        if res < idx {
            idx = res;
            num = *val;
        }
    }

    if num != 0 {
        string.insert_str(idx, i32::to_string(&num).as_str());
    }
    return string;
}

fn replace_last(numbers: HashMap<&str, i32>, mut string: String) -> String {
    let mut idx = usize::MIN;
    let mut num = 0;
    for (key, val) in &numbers {
        let res = string.rfind(key).unwrap_or(usize::MIN);
        if res > idx {
            idx = res;
            num = *val;
        }
    }

    if num != 0 {
        string.insert_str(idx, i32::to_string(&num).as_str());
    }
    return string;
}

fn main() {
    // note the filepath is relative to the run dir!
    let filecontents = fs::read_to_string("./input.txt").expect("failed to read file :(");

    let mut total = 0;

    for line in filecontents.lines() {
        // Create a hash map of strings to numbers like one -> 1 etc

        let mut numbers = HashMap::new();
        let values = "one two three four five six seven eight nine";
        let mut val = 1;
        for key in values.split_whitespace() {
            numbers.insert(key, val);
            val += 1;
        }

        // Replace first and last number words
        let mut filtered = replace_first(numbers.clone(), line.to_string());
        filtered = replace_last(numbers.clone(), filtered);

        // filter
        let chars: Vec<char> = filtered.chars().filter(|x| x.is_digit(10)).collect();

        let val =
            chars[0].to_digit(10).unwrap() * 10 + chars[chars.len() - 1].to_digit(10).unwrap();
        // println!("{}", val);
        total += val;
    }

    println!("{}", total)
}
