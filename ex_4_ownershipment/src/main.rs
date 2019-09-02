fn print_string(str: String) {

    println!("{}",str);
}


fn return_ownership() -> String {

    String::from("Return String")
}


fn pass_ownership(str: String) -> String {
    str
}


fn get_string_len(s: String) -> (String, usize) {

    let len = s.len();
    (s, len)
}

fn get_string_len_by_ref(s: &String) -> usize {
    s.len()
}


fn mutable_ref_fnc(s: &mut String) {
    s.push_str(" ... added ...");
}


//fn dangle() -> &String {
//    let s = String::from("dangling string");
//    &s
//}


fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


fn printout_u32_slices(slc: &[u32]) {
    for &item in slc {
        println!("{}",item)
    }
}


fn main() {
    let s1 = String::from("this is a string");
//    let s2 = s1;
    let s2 = s1.clone();

    println!("{} {}", s1, s2);


    let str = String::from("Hello");
    print_string(str);

    println!("{}", return_ownership());


    println!("{}", pass_ownership(String::from("just passing")));


    let (s3, len) = get_string_len(String::from("Long String"));
    println!("{} {}", s3, len);

    let s4 = String::from("Fourth string");
    let len2 = get_string_len_by_ref(&s4);
    println!("{} {}", s4, len2);

    let mut s5 = String::from("Some string");
    mutable_ref_fnc(&mut s5);
    println!("{}", s5);


    let s6 = String::from("there are many words in this literal");
    println!("{}", first_word(&s6));
    println!("{}", first_word(&s6[..]));

    let s7 = "This is a long tring literal";
    println!("{}", first_word(&s7));


    let u32_array = [1, 2, 3, 4, 5];
    printout_u32_slices(&u32_array[1..3]);
}
