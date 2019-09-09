use std::collections::HashMap;

enum CharType {
    Vowel,
    NonVowel
}

fn is_vowel(c :char) -> CharType {
    match c {
        'a' => CharType::Vowel,
        'e' => CharType::Vowel,
        'i' => CharType::Vowel,
        'o' => CharType::Vowel,
        'u' => CharType::Vowel,
        'y' => CharType::Vowel,
        _ => CharType::NonVowel
    }
}

fn main() {

    // Number Vector

    let number_vector = vec![1, 5, 3, 7, 3, 8, 6, 0, 4, 9, 1, 3, 7, 5, 7, 8, 4, 6, 3, 7, 0];

    let mut number_sum: f64 = 0.0;
    for number in &number_vector {
        number_sum += *number as f64;
    }
    let number_avg =  number_sum / number_vector.len() as f64;
    println!("Average: {}", number_avg);


    let mut number_frequency = HashMap::new();
    for number in &number_vector {
        let key = format!("{}", number);
        let num_count = number_frequency.entry(key).or_insert(0);
        *num_count += 1;
    }

    println!("{:?}", number_frequency);

    let mut sorted_vector = number_vector.clone();
    sorted_vector.sort();
    let median = sorted_vector[sorted_vector.len()/2];
    println!("{:?}", sorted_vector);
    println!("median: {}", median);




    // Pig Latin
    let input_text = String::from("How do you do you devil guy");
    let mut output_text = String::from("");

    for word in input_text.split_whitespace() {
        let initial = word.chars().nth(0).unwrap();
        match is_vowel(initial) {
            CharType::NonVowel => {
                let new_word = format!("{}-{}ay ", word[1..].to_string(), initial);
                output_text.push_str(&new_word);
            },
            CharType::Vowel => {
                let new_word = format!("{}-Hay ", word);
                output_text.push_str(&new_word);
            }
        }
    }

    println!("{}", output_text);



    // Text processor
    let cmds = vec![
        "Add Samuel to Developers",
        "Add John to Managers",
        "Add Eve to Developers",
    ];


    let mut employ_table : HashMap<String, Vec<String>> = HashMap::new();
    for cmd in cmds {
        let parsed_cmd = process_cmd(cmd);
        let departament = employ_table.entry(parsed_cmd.departament).or_insert(vec![]);
        if !parsed_cmd.error {
            departament.push(parsed_cmd.name);
        }

    }

    println!("{:?}", employ_table);
}

struct ParsedAddEmployCmd {
    name : String,
    departament : String,
    error : bool
}

impl ParsedAddEmployCmd {
    fn new(n : &str, d : &str, e : bool) -> ParsedAddEmployCmd {
        ParsedAddEmployCmd {
            name: n.to_string(),
            departament: d.to_string(),
            error: e
        }
    }
}

enum StateMachine {
    Init,
    Add,
    Name,
    To,
    Departament
}

fn process_cmd(s: &str) ->ParsedAddEmployCmd {
    let mut name = "";
    let mut dep = "";
    let mut err = true;
    let mut state = StateMachine::Init;
    for word in s.split_whitespace() {
        match state {
            StateMachine::Init => {state = StateMachine::Add;},
            StateMachine::Add => {name = word; state = StateMachine::Name;},
            StateMachine::Name => {state = StateMachine::To;},
            StateMachine::To => {dep = word; state = StateMachine::Departament},
            _ => {}
        };
    }

    match state {
        StateMachine::Departament => err = false,
        _ => {}
    };

    ParsedAddEmployCmd::new(name, dep, err)
}
