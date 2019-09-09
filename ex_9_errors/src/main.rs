use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {

    let file_path = String::from("hello.txt");
//    let f = File::open(&file_path).unwrap();
    let f = File::open(&file_path).expect("Unable to open file");

//    let f = match f {
//        Ok(file) => file,
//        Err(ref error) if error.kind() == ErrorKind::NotFound => {
//            match File::create(&file_path) {
//                Ok(fc) => {
//                    println!("File just created");
//                    fc
//                }
//
//                Err(e) => {
//                    panic!("Triend to create file but there was a problem: {:?}", e);
//                }
//            }
//        },
//        Err(error) => {
//            panic!("There was a problem opening the file: {:?}", error);
//        }
//    };

}


fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}


fn read_user_from_file_shortcut() -> Result<String, io::Error> {

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_super_short() -> Result<String, io::Error> {
    let mut s = String::new();
    let f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


pub struct Guess {
    value : u32
}

impl Guess {

    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value {} out of range", value);
        }
        Guess {value}
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}