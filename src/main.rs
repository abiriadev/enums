use crate::UsState::Connecticut;

enum IPaddrType {
    v4,
    v6,
}

enum newIPaddrType {
    v4(u8, u8, u8, u8),
    v6(String),
}

struct IPaddr {
    kind: IPaddrType,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

struct QuitMessage;

struct WriteMessage(String);

enum myOption<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    Rhode,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    West,
    Wisconsin,
    Wyoming,
}

fn main() {
    println!("Hello, world!");

    old_enum();

    new_enum();

    message();

    option();
    // route(IPaddrType::v4);
    let value = value_in_cents(Coin::Penny);
    let value = value_in_cents(Coin::Quarter(UsState::Alaska));

    if_let(0u8);
    if_let2(3u8);

    let mut count = 0;

    let coin = Coin::Quarter(UsState::California);

    let coin_ref: &Coin = &coin;
    let coin_ref2: &Coin = &coin;

    match coin_ref {
        Coin::Quarter(state) => println!("25cent coin of {:?}", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin_ref2 {
        println!("25cent coin of {:?}", state)
    } else {
        count += 1
    }
}

fn if_let2(number_u8: u8) {
    let some_u8 = Some(number_u8);

    if let Some(3) = some_u8 {
        println!("three");
    }
}

fn if_let(number_u8: u8) {
    let some_u8 = Some(number_u8);

    match some_u8 {
        Some(3) => println!("three!"),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickle => 25,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state Quarter from {:?}", state);
            25
        }
    }
}

fn option() {
    let some_number: Option<i32> = Option::Some(454300);
    let some_string: Option<&str> = Some("a string");

    let absent_string: Option<i32> = Option::None;

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn message() {
    let msg = Message::Write(String::from("Hello, world!"));
    let msg2 = Message::Move {
        x: 454300,
        y: 12808,
    };

    let msg3 = Message::Quit;
    let msg4 = Message::ChangeColor(231, 2341, 1_2314);

    msg.call();
    msg2.call();
    msg3.call();
    msg4.call();
}

fn new_enum() {
    let localhost = newIPaddrType::v4(127, 0, 0, 1);
    let loopback = newIPaddrType::v6(String::from("::127"));
}

fn old_enum() {
    let four = IPaddrType::v4;
    let six = IPaddrType::v6;

    let localhost = IPaddr {
        kind: IPaddrType::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IPaddr {
        kind: IPaddrType::v6,
        address: String::from("::1"),
    };

    route(localhost.kind);
    route(loopback.kind);
}

fn route(ip_type: IPaddrType) {
    match ip_type {
        IPaddrType::v4 => println!("V4!"),
        IPaddrType::v6 => println!("V6!"),
    }
}
