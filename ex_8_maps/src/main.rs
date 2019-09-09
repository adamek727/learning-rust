use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);


    let teams = vec![String::from("blue"), String::from("red")];
    let scrs = vec![30, 40];

    let scores: HashMap<_, _> = teams.iter().zip(scrs.iter()).collect();


    let some_key = String::from("key");
    let some_value = 666;

    let mut map = HashMap::new();
    map.insert(some_key, some_value);
//    println!("{} : {}", some_key, some_value); // error

    map.insert(String::from("another_key"), 999);

    for (key, val) in map {
        println!("{} {}", key, val);
    }


    // Overwriting val
    let mut overwrited = HashMap::new();
    overwrited.insert("super_key", 10);
    overwrited.insert("super_key", 20);
    println!("{:?}", overwrited);

    // Check key exists
    let mut checked = HashMap::new();
    checked.insert(String::from("green"), 50);

    checked.entry(String::from("yellow")).or_insert(80);
    checked.entry(String::from("green")).or_insert(100);
    println!("{:?}", checked);

    // Update value
    let mut word_counter = HashMap::new();
    let sentence = String::from("Hello world you wonderfull world");

    for word in sentence.split_whitespace() {
        let count = word_counter.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_counter);

}
