use std::borrow::BorrowMut;

use crate::command::Command;

fn sum(data:&[u8])->u32{
    let mut a:u32=0;
    for b in data{
        a+=*b as u32;
    }
    a
}

pub fn encode(cmd:&Command,arg1:u32,arg2:u32,data:&[u8])->Vec<u8>{
    let mut buf:Vec<u8>=Vec::new();
    buf.append(cmd.command.to_be_bytes().to_vec().borrow_mut());

    buf.append(arg1.to_le_bytes().to_vec().borrow_mut());
    buf.append(arg2.to_le_bytes().to_vec().borrow_mut());

    buf.append((data.len() as u32).to_le_bytes().to_vec().borrow_mut());
    
    buf.append(sum(data).to_le_bytes().to_vec().borrow_mut());

    buf.append(cmd.magic.to_be_bytes().to_vec().borrow_mut());

    buf
}