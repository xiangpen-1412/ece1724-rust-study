// ECE1724 — Lecture 5: Enums (Book 6.1)
// Pages: 25-36 (Lecture 5 slides; title page = 1).
// Topics: variants, associated data, IpAddr, Message, enum methods.

use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("Route: {ip_kind:?}");
}

// First design: the kind and address are separate struct fields.
struct AddressRecord {
    kind: IpAddrKind,
    address: String,
}

// Second design: the address is part of the enum variant.
enum IpAddr {
    V4(String),
    V6(String),
}

// Improved version: each IPv4 component must fit in a u8.
enum Ip {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Keep the classroom name; Rust normally uses CamelCase for type names.
#[allow(non_camel_case_types)]
enum IpAddr_P {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,                       // No associated data.
    Move { x: i32, y: i32 },    // Named fields, like a struct.
    Write(String),              // One value.
    ChangeColor(i32, i32, i32), // Positional fields, like a tuple struct.
}

impl Message {
    fn call(&self) {
        // A small match preview; see 10_match_patterns.rs for the rules.
        // Matching &self borrows the data, so Write does not move its String.
        match self {
            Self::Quit => println!("Quit"),
            Self::Move { x, y } => println!("Move to ({x}, {y})"),
            Self::Write(text) => println!("Write: {text}"),
            Self::ChangeColor(r, g, b) => println!("Color: ({r}, {g}, {b})"),
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six); // Both variants have the same enum type.

    let records = [
        AddressRecord {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        },
        AddressRecord {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        },
    ];
    for record in records {
        println!("Record: {:?} {}", record.kind, record.address);
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    // Read the stored data with match (covered in 10_match_patterns.rs).
    for address in [home, loopback] {
        match address {
            IpAddr::V4(text) => println!("IPv4 string: {text}"),
            IpAddr::V6(text) => println!("IPv6 string: {text}"),
        }
    }

    for address in [Ip::V4(127, 0, 0, 1), Ip::V6(String::from("::1"))] {
        match address {
            Ip::V4(a, b, c, d) => println!("IPv4 bytes: {a}.{b}.{c}.{d}"),
            Ip::V6(text) => println!("IPv6 string: {text}"),
        }
    }

    for address in [
        IpAddr_P::V4(Ipv4Addr::new(127, 0, 0, 1)),
        IpAddr_P::V6(Ipv6Addr::LOCALHOST),
    ] {
        match address {
            IpAddr_P::V4(ip) => println!("IPv4 struct: {ip}"),
            IpAddr_P::V6(ip) => println!("IPv6 struct: {ip}"),
        }
    }

    let message = Message::Move { x: 10, y: 10 };
    message.call();
    message.call(); // call borrows message, so it remains available.
    // Different variants can be stored together because all are Message.
    for message in [
        Message::Quit,
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ] {
        message.call();
    }
}

// Expand (leave for practice):
// TODO: Add Message::Resize { width, height } and update call for the new variant.
// TODO: Add Ip::is_v4(&self); check V4(127, 0, 0, 1) -> true and V6("::1") -> false.
