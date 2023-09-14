use crate::{message::encode, command::CNXN};

mod message;
mod command;

#[macro_use]
extern crate lazy_static;

fn main() {
    let data="host::\0";
    let arr=data.as_bytes();
    let buf=encode(&CNXN, 0x01000000, 0x1000, arr);
    println!("{:?}",buf);
    println!("{:?}",arr);
}
