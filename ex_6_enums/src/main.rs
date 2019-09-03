enum IpAddrType {
    V4,
    V6
}

struct IpAddr {
    typ: IpAddrType,
    addr: String
}

enum IpAddrAssociated {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeCollor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Texas,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let ipv4 = IpAddrType::V4;
    let ipv6 = IpAddrType::V6;

    route(ipv4);
    route(ipv6);


    let localhost = IpAddr {
        typ: IpAddrType::V4,
        addr: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        typ: IpAddrType::V6,
        addr: String::from("::1")
    };

    let remote = IpAddrAssociated::V4(8,8,8,8);
    let msg = Message::Write(String::from("hello"));
    msg.call();


    let coin = Coin::Quarter(UsState::California);
    let coin_val = value_in_cents(coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);



    let coin2 = Coin::Nickel;
    if let Coin::Nickel = coin2 {
        println!{"This is a nickel"};
    }

    match coin2 {
        Coin::Nickel => println!("This is a nickel"),
        _ => (),
    };
}


fn route(addr: IpAddrType) {

}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Dime => 5,
        Coin::Nickel => 10,
        Coin::Quarter(state) => {
            println!("Quarter by: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}