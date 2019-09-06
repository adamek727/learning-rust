
enum DataCells {
    Int32(i32),
    Float(f64),
    Str(String)
}


fn main() {

    let explicit_vector: Vec<i32> = Vec::new();
    let implicit_vector = vec![1, 2, 3];


    let mut mutable_vector = Vec::new();

    mutable_vector.push(10);
    mutable_vector.push(11);
    mutable_vector.push(12);

    let vector_second_val = mutable_vector[1];
    let vector_second_val_ref = &mutable_vector[1];
    let vector_third_val = mutable_vector.get(2);

    println!("{} {} {:?}", vector_second_val, vector_second_val_ref ,vector_third_val);


//    let vector_non_exist_val = mutable_vector[1000]; // crash
//    let vector_non_exist_val_ref = &mutable_vector[1000]; // crash
    let vector_non_exist_get = &mutable_vector.get(1000);

    println!("{:?}" ,vector_non_exist_get);


    let first_element = &mutable_vector[0];
//    mutable_vector.push(13);
    println!("{}", first_element);


    let mut second_mut_vec = vec![21, 22, 23];
    for val in &mut second_mut_vec { println!("{}", val); }
    for val in &mut second_mut_vec { *val += 10; }
    for val in second_mut_vec { println!("{}", val); }


    let mixed_vector = vec![DataCells::Int32(666),
                                          DataCells::Float(66.6),
                                          DataCells::Str(String::from("666"))];
}
