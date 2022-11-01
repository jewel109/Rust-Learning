#[derive(Debug)]

enum IpAddr {
    v4,
    v6,
}
struct Addr {
    kind: IpAddr,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body goes here
    }
}

fn main() {
    let absent_number: Option<i32> = None;

    let home = Addr {
        kind: IpAddr::v4,
        address: String::from("127.0.0.1"),
    };

    let x:i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; it will give error
    let m = Message::Write(String::from("hei"));
    m.call()

    //println!("home is {:#?}", home)
}
