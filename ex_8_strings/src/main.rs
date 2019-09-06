fn main() {
    let mut s= String::new();

    let data = "some data string";

    let string_from_data_1 = data.to_string();
    let string_from_data_2 = String::from(data);


    let mut short_str = "foo".to_string();
    short_str.push_str(" bublebum");


    let mut s1 = "foo".to_string();
    let s2 = "bar";
    s1.push_str(s2);
    println!("{} {}",s1, s2);


    let t1 = String::from("bu");
    let t2 = String::from("ble");
    let t3 = String::from("gum");
    let bublegum = t1+&t2+&t3;
//    let bublegum_2 = format!("{}{}{}",t1,t2,t3);
    println!("{}", bublegum);

    let hello = "zdrastvujte";
    let z = &hello[0..5]; // possible crash


    for x in hello.chars() {
        println!("{}",x)
    }

    for x in hello.bytes() {
        println!("{}",x)
    }
}
